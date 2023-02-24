use std::mem;

use glam::{vec3, Mat4};
use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Context, CullFace, EventHandler,
    PassAction, Pipeline, Shader, ShaderMeta, UniformBlockLayout, UniformDesc, VertexAttribute,
    VertexFormat, VertexStep,
};
use std::f32::consts::PI;

use crate::{
    math::{Vector2, Vector3},
    simulation::{OwningRun, Simulation},
};

#[repr(C)]
struct Vertex<T> {
    pos: T,
}

pub fn new_conf() -> Conf {
    Conf {
        high_dpi: true,
        ..Default::default()
    }
}

pub struct Stage<const N: usize> {
    pipeline: Pipeline,
    bindings: Bindings,
    num_indices: usize,
    scale: f32,
    run: OwningRun<N>,
    inst_pos: Vec<Vector3>,
    inst_scale: Vec<Vector3>,
    ry: f32,
}

#[allow(dead_code)]
struct Uniforms {
    mvp: Mat4,
}

const BODY_WIDTHS: [f32; 2] = [6_378_000.0, 100_000.0];

impl<const N: usize> EventHandler for Stage<N> {
    fn update(&mut self, _ctx: &mut Context) {
        let step = self.run.next().unwrap();

        for (i, (_, body)) in step.body_map.iter().enumerate() {
            self.inst_pos[i][0] = body.position[0] / (self.scale);
            self.inst_pos[i][1] = body.position[1] / (self.scale);
            self.inst_pos[i][2] = body.position[2] / (self.scale);
            self.inst_scale[i][0] = BODY_WIDTHS[i] / (self.scale);
            self.inst_scale[i][1] = BODY_WIDTHS[i] / (self.scale);
            self.inst_scale[i][2] = BODY_WIDTHS[i] / (self.scale);
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.bindings.vertex_buffers[1].update(ctx, &self.inst_pos[..]);
        self.bindings.vertex_buffers[2].update(ctx, &self.inst_scale[..]);

        // model-view-projection matrix
        let (width, height) = ctx.screen_size();

        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 50.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.ry += 0.01;
        let mvp = view_proj * Mat4::from_rotation_y(self.ry);

        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0., 0., 0., 1.)),
            depth: None,
            stencil: None,
        });

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&Uniforms { mvp });

        ctx.draw(0, self.num_indices as i32, self.inst_pos.len() as i32);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

const VERTEX_SHADER: &str = include_str!("shaders/geo.vert");
const FRAGMENT_SHADER: &str = include_str!("shaders/geo.frag");

#[allow(dead_code)]
fn generate_circle(segments: u32) -> (Vec<Vertex<Vector2>>, Vec<u32>) {
    let mut vertices = vec![];
    let mut indices = vec![];
    vertices.push(Vertex {
        pos: Vector2::from([0.0, 0.0]),
    });
    vertices.push(Vertex {
        pos: Vector2::from([1.0, 0.0]),
    });
    for i in 1..segments {
        let radians = i as f32 * 2.0 * PI / segments as f32;
        vertices.push(Vertex {
            pos: Vector2::from([radians.cos(), radians.sin()]),
        });
        indices.extend(vec![0, i + 1, i]);
    }
    indices.extend(vec![0, 1, segments]);
    (vertices, indices)
}

fn generate_uv_sphere(n_stacks: u32, n_sectors: u32) -> (Vec<Vertex<Vector3>>, Vec<u32>) {
    let mut vertices = vec![];
    let mut indices = vec![];

    // First create bottom and top points
    vertices.push(Vertex {
        pos: Vector3::new(0.0, -1.0, 0.0),
    });
    vertices.push(Vertex {
        pos: Vector3::new(0.0, 1.0, 0.0),
    });

    for stack_step in 1..n_stacks {
        let phi = -PI / 2.0 + PI * stack_step as f32 / n_stacks as f32;

        // Create n_sectors+1. The first and last will have the same position coordinates
        // but allow for different texture coordinates.
        for sector_step in 0..=n_sectors {
            let theta = 2.0 * PI * sector_step as f32 / n_sectors as f32;
            let z_proj_magnitude = phi.cos();

            vertices.push(Vertex {
                pos: Vector3::new(
                    z_proj_magnitude * theta.cos(),
                    phi.sin(),
                    z_proj_magnitude * theta.sin(),
                ),
            })
        }
    }

    // Bottom and top stacks will only need single triangles for their faces
    for sector_step in 0..n_sectors {
        let bottom_vertex_offset = 2 + sector_step;
        let top_vertex_offset = 2 + (n_sectors + 1) * (n_stacks - 2) + sector_step;
        indices.extend_from_slice(&[bottom_vertex_offset, 0, bottom_vertex_offset + 1]);
        indices.extend_from_slice(&[top_vertex_offset, 1, top_vertex_offset + 1]);
    }

    // Each sector of each stack will require two triangles to cover their quadrangle
    let vertices_per_stack = n_sectors + 1;
    for stack_step in 1..(n_stacks - 1) {
        let num_stacks_below = stack_step - 1;
        let vertex_offset = num_stacks_below * vertices_per_stack;
        for sector_step in 0..n_sectors {
            let bottom_left_point = 2 + vertex_offset + sector_step;
            let bottom_right_point = bottom_left_point + 1;
            let top_left_point = bottom_left_point + vertices_per_stack;
            let top_right_point = top_left_point + 1;
            indices.extend_from_slice(&[top_left_point, bottom_left_point, top_right_point]);
            indices.extend_from_slice(&[top_right_point, bottom_left_point, bottom_right_point]);
        }
    }

    (vertices, indices)
}

impl<const N: usize> Stage<N> {
    const MAX_BODIES: usize = 256;

    pub fn new(context: &mut Context, simulation: Simulation<N>) -> Self {
        let (vertices, indices) = generate_uv_sphere(10, 12);
        let geometry_vertex_buffer =
            Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &indices);

        let positions_vertex_buffer = Buffer::stream(
            context,
            BufferType::VertexBuffer,
            Self::MAX_BODIES * mem::size_of::<Vector3>(),
        );

        let scale_vertex_buffer = Buffer::stream(
            context,
            BufferType::VertexBuffer,
            Self::MAX_BODIES * mem::size_of::<Vector3>(),
        );

        let bindings = Bindings {
            vertex_buffers: vec![
                geometry_vertex_buffer,
                positions_vertex_buffer,
                scale_vertex_buffer,
            ],
            index_buffer: index_buffer,
            images: vec![],
        };

        let meta = ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", miniquad::UniformType::Mat4)],
            },
        };
        let shader = Shader::new(context, VERTEX_SHADER, FRAGMENT_SHADER, meta).unwrap();
        let pipeline = Pipeline::new(
            context,
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("pos", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("inst_pos", VertexFormat::Float3, 1),
                VertexAttribute::with_buffer("inst_scale", VertexFormat::Float3, 2),
            ],
            shader,
        );

        context.set_cull_face(CullFace::Nothing);

        let mut inst_pos = Vec::with_capacity(Self::MAX_BODIES);
        inst_pos.resize(simulation.bodies().len(), Vector3::default());
        let mut inst_scale = Vec::with_capacity(Self::MAX_BODIES);
        inst_scale.resize(simulation.bodies().len(), Vector3::default());

        let run = OwningRun::from(simulation);
        Self {
            pipeline,
            bindings,
            scale: 10_000_000.0,
            num_indices: indices.len(),
            run,
            inst_pos,
            inst_scale,
            ry: 0.0,
        }
    }
}
