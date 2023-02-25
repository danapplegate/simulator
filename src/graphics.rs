use glam::{vec3, Mat4, Vec3};
use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Context, CullFace, EventHandler,
    PassAction, Pipeline, Shader, ShaderMeta, UniformBlockLayout, UniformDesc, UniformType,
    VertexAttribute, VertexFormat, VertexStep,
};
use std::mem;

use crate::{
    math::Vector3,
    simulation::{OwningRun, Simulation},
};

pub mod model;
use model::generate_uv_sphere;

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
    light_color: Vec3,
    light_pos: Vec3,
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

        let light_color = vec3(1.0, 1.0, 1.0);
        let light_pos = vec3(-2.0, 2.0, 4.0);

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
        ctx.apply_uniforms(&Uniforms {
            mvp,
            light_color,
            light_pos,
        });

        ctx.draw(0, self.num_indices as i32, self.inst_pos.len() as i32);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

const VERTEX_SHADER: &str = include_str!("shaders/geo.vert");
const FRAGMENT_SHADER: &str = include_str!("shaders/geo.frag");

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
                uniforms: vec![
                    UniformDesc::new("mvp", UniformType::Mat4),
                    UniformDesc::new("light_color", UniformType::Float3),
                ],
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
                VertexAttribute::with_buffer("normal", VertexFormat::Float3, 0),
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
