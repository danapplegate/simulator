use glam::{vec3, Mat4, Quat, Vec3};
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
    ry: f32,
}

#[allow(dead_code)]
struct Uniforms {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    light_color: Vec3,
    light_pos: Vec3,
}

const BODY_WIDTHS: [f32; 2] = [6_378_000.0, 100_000.0];

impl<const N: usize> EventHandler for Stage<N> {
    fn update(&mut self, _ctx: &mut Context) {
        let step = self.run.next().unwrap();

        for (i, (_, body)) in step.body_map.iter().enumerate() {
            self.inst_pos[i][0] = body.position[0];
            self.inst_pos[i][1] = body.position[1];
            self.inst_pos[i][2] = body.position[2];
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // model-view-projection matrix
        let (width, height) = ctx.screen_size();

        let light_color = vec3(1.0, 1.0, 1.0);
        let light_pos = vec3(-2.0, 2.0, 4.0);

        self.ry += 0.01;
        let view = Mat4::look_at_rh(
            vec3(0.0, self.scale / 4.0, 1.5 * self.scale),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        ) * Mat4::from_rotation_y(self.ry);

        let projection =
            Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 2.0 * self.scale);

        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0., 0., 0., 1.)),
            depth: None,
            stencil: None,
        });

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        for i in 0..self.inst_pos.len() {
            let inst_pos = &self.inst_pos[i];
            let inst_scale = BODY_WIDTHS[i];
            let model = Mat4::from_scale_rotation_translation(
                inst_scale * Vec3::ONE,
                Quat::from_rotation_x(0.0),
                vec3(inst_pos.x(), inst_pos.y(), inst_pos.z()),
            );

            ctx.apply_uniforms(&Uniforms {
                model,
                view,
                projection,
                light_color,
                light_pos,
            });

            ctx.draw(0, self.num_indices as i32, 1);
        }

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

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let meta = ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("view", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
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
            ],
            shader,
        );

        context.set_cull_face(CullFace::Nothing);

        let mut inst_pos = Vec::with_capacity(Self::MAX_BODIES);
        inst_pos.resize(simulation.bodies().len(), Vector3::default());

        let run = OwningRun::from(simulation);
        Self {
            pipeline,
            bindings,
            scale: 10_000_000.0,
            num_indices: indices.len(),
            run,
            inst_pos,
            ry: 0.0,
        }
    }
}
