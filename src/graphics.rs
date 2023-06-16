use glam::{vec3, Mat4, Quat, Vec3};
use image::io::Reader as ImageReader;
use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Comparison, Context, CullFace,
    EventHandler, FilterMode, KeyCode, PassAction, Pipeline, PipelineParams, Shader, ShaderMeta,
    Texture, TextureFormat, TextureParams, TextureWrap, UniformBlockLayout, UniformDesc,
    UniformType, VertexAttribute, VertexFormat, VertexStep,
};

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
    inst_rot: Vec<Vector3>,
    ry: f32,
    rx: f32,
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
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: miniquad::KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => self.rx += 0.1,
            KeyCode::Down => self.rx -= 0.1,
            KeyCode::Left => self.ry -= 0.1,
            KeyCode::Right => self.ry += 0.1,
            _ => {}
        }
    }

    fn update(&mut self, _ctx: &mut Context) {
        let step = self.run.next().unwrap();

        for (i, (_, body)) in step.body_map.iter().enumerate() {
            self.inst_pos[i][0] = body.position[0];
            self.inst_pos[i][1] = body.position[1];
            self.inst_pos[i][2] = body.position[2];
            self.inst_rot[i][1] += 0.01;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // model-view-projection matrix
        let (width, height) = ctx.screen_size();

        let light_color = vec3(1.0, 1.0, 1.0);
        let light_pos = vec3(-2.0, 2.0, 4.0);

        let view = Mat4::look_at_rh(
            vec3(0.0, 0.0, 2.5),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        ) * Mat4::from_rotation_y(self.ry)
            * Mat4::from_rotation_x(self.rx);

        let projection = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);

        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0., 0., 0., 0.)),
            depth: Some(1.),
            stencil: None,
        });

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        for i in 0..self.inst_pos.len() {
            let inst_pos = &self.inst_pos[i];
            let inst_rot = &self.inst_rot[i];
            let inst_scale = BODY_WIDTHS[i];
            let model = Mat4::from_scale_rotation_translation(
                inst_scale * Vec3::ONE / self.scale,
                Quat::from_rotation_y(inst_rot[1]),
                vec3(
                    inst_pos.x() / self.scale,
                    inst_pos.y() / self.scale,
                    inst_pos.z() / self.scale,
                ),
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
        let (vertices, indices) = generate_uv_sphere(20, 24);
        let geometry_vertex_buffer =
            Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &indices);

        let img = ImageReader::open("data/earth.jpeg")
            .unwrap()
            .decode()
            .unwrap();
        let tex = Texture::from_data_and_format(
            context,
            img.as_rgb8().unwrap(),
            TextureParams {
                format: TextureFormat::RGB8,
                wrap: TextureWrap::Repeat,
                filter: FilterMode::Linear,
                width: img.width(),
                height: img.height(),
            },
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![tex],
        };

        let meta = ShaderMeta {
            images: vec!["textureSource".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("view", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                    UniformDesc::new("light_color", UniformType::Float3),
                    UniformDesc::new("light_pos", UniformType::Float3),
                ],
            },
        };
        let shader = Shader::new(context, VERTEX_SHADER, FRAGMENT_SHADER, meta).unwrap();
        let mut pipeline_params = PipelineParams::default();
        pipeline_params.depth_test = Comparison::LessOrEqual;
        pipeline_params.depth_write = true;
        let pipeline = Pipeline::with_params(
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
                VertexAttribute::with_buffer("tex_coord", VertexFormat::Float2, 0),
            ],
            shader,
            pipeline_params,
        );

        context.set_cull_face(CullFace::Nothing);

        let mut inst_pos = Vec::with_capacity(Self::MAX_BODIES);
        inst_pos.resize(simulation.bodies().len(), Vector3::default());
        let mut inst_rot = Vec::with_capacity(Self::MAX_BODIES);
        inst_rot.resize(simulation.bodies().len(), Vector3::default());

        let run = OwningRun::from(simulation);
        Self {
            pipeline,
            bindings,
            scale: 10_000_000.0,
            num_indices: indices.len(),
            run,
            inst_pos,
            inst_rot,
            ry: 0.0,
            rx: 0.0,
        }
    }
}
