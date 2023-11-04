use glam::{vec3, Mat4};
use miniquad::{
    conf::Conf, BufferLayout, Comparison, Context, CullFace, EventHandler, KeyCode, MouseButton,
    PassAction, Pipeline, PipelineParams, Shader, ShaderMeta, UniformBlockLayout, UniformDesc,
    UniformType, VertexAttribute, VertexFormat, VertexStep,
};
use std::{cmp::min, collections::HashMap, path::PathBuf};

use crate::{
    math::Vector3,
    simulation::{Body, OwningRun, Simulation},
};

pub mod model;
pub mod trackball;
use self::model::{Model, Uniforms};
use self::trackball::Trackball;

pub fn new_conf() -> Conf {
    Conf {
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Default)]
pub struct BodyState {
    pos: Vector3,
    rot: f32,
    diameter: f32,
    tilt: f32,
}
impl From<&Body<3>> for BodyState {
    fn from(body: &Body<3>) -> Self {
        BodyState {
            pos: body.position,
            rot: body.spin.angle,
            diameter: body.diameter,
            tilt: body.spin.tilt,
        }
    }
}

pub type BodyStateMap = HashMap<String, BodyState>;

pub struct Stage {
    pipeline: Pipeline,
    scale: f32,
    run: OwningRun<3>,
    body_state_map: BodyStateMap,
    ry: f32,
    rx: f32,
    models: HashMap<String, Model>,
    trackball: Trackball,
}

// Converts screen coordinates to normalized device coordinates [-1, 1]
fn normalize(x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
    let s: f32 = min(width as i32, height as i32) as f32 - 1.0;
    ((2.0 * x - width + 1.0) / s, (2.0 * y - height + 1.0) / s)
}

impl EventHandler for Stage {
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let MouseButton::Left = button {
            let (width, height) = ctx.screen_size();
            let (nx, ny) = normalize(x, y, width, height);
            self.trackball.start(nx, ny);
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        if self.trackball.is_active() {
            let (width, height) = ctx.screen_size();
            let (nx, ny) = normalize(x, y, width, height);
            self.trackball.set_xy(nx, ny);
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if let MouseButton::Left = button {
            if self.trackball.is_active() {
                let (width, height) = ctx.screen_size();
                let (nx, ny) = normalize(x, y, width, height);
                self.trackball.end(nx, ny);
            }
        }
    }

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

        for (label, body) in step.body_map.iter() {
            self.body_state_map.get_mut(label).unwrap().pos = body.position;
            self.body_state_map.get_mut(label).unwrap().rot = body.spin.angle;
            self.body_state_map.get_mut(label).unwrap().diameter = body.diameter;
            self.body_state_map.get_mut(label).unwrap().tilt = body.spin.tilt;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // model-view-projection matrix
        let (width, height) = ctx.screen_size();

        let light_color = vec3(1.0, 1.0, 1.0);
        let light_pos = vec3(-2.0, 2.0, 4.0);

        let view_rot = self.trackball.view_rotation();

        let view = Mat4::look_at_rh(
            vec3(0.0, 0.0, 0.3),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        ) * Mat4::from_quat(view_rot);

        let projection =
            Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 1_000_000.0);

        let mut uniforms = Uniforms::default();
        uniforms.view = view;
        uniforms.projection = projection;
        uniforms.light_color = light_color;
        uniforms.light_pos = light_pos;

        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0., 0., 0., 0.)),
            depth: Some(1.),
            stencil: None,
        });

        ctx.apply_pipeline(&self.pipeline);

        for (_, m) in &self.models {
            m.draw_bodies(ctx, &self.body_state_map, &mut uniforms, self.scale);
        }

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

const VERTEX_SHADER: &str = include_str!("shaders/geo.vert");
const FRAGMENT_SHADER: &str = include_str!("shaders/geo.frag");

impl Stage {
    const MAX_BODIES: usize = 256;

    pub fn new(
        context: &mut Context,
        simulation: Simulation<3>,
        mut models: HashMap<String, Model>,
        config_root: PathBuf,
    ) -> Self {
        for (_, m) in &mut models {
            m.load(context, &config_root);
        }

        let meta = ShaderMeta {
            images: vec!["textureSource".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("view", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                    UniformDesc::new("normal_mat", UniformType::Mat4),
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

        context.set_cull_face(CullFace::Back);

        let mut inst_pos = Vec::with_capacity(Self::MAX_BODIES);
        inst_pos.resize(simulation.bodies().len(), Vector3::default());
        let mut inst_rot = Vec::with_capacity(Self::MAX_BODIES);
        inst_rot.resize(simulation.bodies().len(), Vector3::default());

        let mut body_state_map = BodyStateMap::new();
        for b in simulation.bodies() {
            body_state_map.insert(b.label.clone(), b.into());
        }

        let run = OwningRun::from(simulation);

        Self {
            pipeline,
            body_state_map,
            scale: 100_000_000.0,
            run,
            ry: 0.0,
            rx: 0.0,
            models: models,
            trackball: Trackball::default(),
        }
    }
}
