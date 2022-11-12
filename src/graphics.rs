use miniquad::{
    conf::Conf, Bindings, Buffer, BufferType, Context, EventHandler, PassAction, Pipeline, Shader,
    ShaderMeta, UniformBlockLayout,
};

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

pub struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.7, 0., 1., 1.)),
            depth: None,
            stencil: None,
        });

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.draw(0, 1, 1);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

const VERTEX_SHADER: &str = r#"
    #version 100

    attribute vec2 pos;

    void main() {
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 100

    void main() {
        gl_FragColor = vec4(0.0, 0.7, 0.7, 1.0);
    }
"#;

impl Stage {
    pub fn new(context: &mut Context) -> Self {
        let meta = ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        };
        let shader = Shader::new(context, VERTEX_SHADER, FRAGMENT_SHADER, meta).unwrap();
        let pipeline = Pipeline::new(context, &vec![], &vec![], shader);
        let vertices = [
            Vertex {
                pos: Vec2 { x: -0.25, y: 0.25 },
            },
            Vertex {
                pos: Vec2 { x: 0.25, y: 0.0 },
            },
            Vertex {
                pos: Vec2 { x: 0.0, y: -0.25 },
            },
        ];
        let vertex_buffer = Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &[0, 1, 2]);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };
        Self { pipeline, bindings }
    }
}
