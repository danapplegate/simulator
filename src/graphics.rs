use std::mem;

use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Context, EventHandler, PassAction,
    Pipeline, Shader, ShaderMeta, UniformBlockLayout, VertexAttribute, VertexFormat, VertexStep,
};

use crate::simulation::{OwningRun, Simulation};

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
}

pub fn new_conf() -> Conf {
    Conf {
        ..Default::default()
    }
}

pub struct Stage<const N: usize> {
    pipeline: Pipeline,
    bindings: Bindings,
    run: OwningRun<N>,
    inst_pos: Vec<Vec2>,
}

impl<const N: usize> EventHandler for Stage<N> {
    fn update(&mut self, _ctx: &mut Context) {
        let step = self.run.next().unwrap();

        if (self.inst_pos.is_empty()) {
            for (_, body) in step.body_map {
                self.inst_pos.push(Vec2 {
                    x: body.position[0] as f32 / 10_000_000.0,
                    y: body.position[1] as f32 / 10_000_000.0,
                });
            }
        } else {
            for (i, (_, body)) in step.body_map.iter().enumerate() {
                self.inst_pos[i].x = body.position[0] as f32 / 10_000_000.0;
                self.inst_pos[i].y = body.position[1] as f32 / 10_000_000.0;
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.bindings.vertex_buffers[1].update(ctx, &self.inst_pos[..]);

        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.7, 0., 1., 1.)),
            depth: None,
            stencil: None,
        });

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.draw(0, 3, self.inst_pos.len() as i32);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

const VERTEX_SHADER: &str = r#"
    #version 100

    attribute vec2 pos;
    attribute vec2 inst_pos;

    void main() {
        gl_Position = vec4(pos + inst_pos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 100

    void main() {
        gl_FragColor = vec4(0.0, 0.7, 0.7, 1.0);
    }
"#;

impl<const N: usize> Stage<N> {
    const MAX_BODIES: usize = 256;

    pub fn new(context: &mut Context, simulation: Simulation<N>) -> Self {
        let vertices = [
            Vertex {
                pos: Vec2 { x: 0.0, y: 0.05 },
            },
            Vertex {
                pos: Vec2 { x: 0.05, y: -0.05 },
            },
            Vertex {
                pos: Vec2 { x: -0.05, y: -0.05 },
            },
        ];
        let geometry_vertex_buffer =
            Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &[0, 1, 2]);

        let positions_vertex_buffer = Buffer::stream(
            context,
            BufferType::VertexBuffer,
            Self::MAX_BODIES * mem::size_of::<Vec2>(),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let meta = ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
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
            ],
            &[
                VertexAttribute::with_buffer("pos", VertexFormat::Float2, 0),
                VertexAttribute::with_buffer("inst_pos", VertexFormat::Float2, 1),
            ],
            shader,
        );

        let run = OwningRun::from(simulation);
        Self {
            pipeline,
            bindings,
            run,
            inst_pos: Vec::with_capacity(Self::MAX_BODIES),
        }
    }
}
