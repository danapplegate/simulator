use glam::{vec3, Mat4, Quat, Vec3};
use image::io::Reader as ImageReader;
use miniquad::{
    Bindings, Buffer, BufferType, Context, FilterMode, Texture, TextureFormat, TextureParams,
    TextureWrap,
};
use serde::{Deserialize, Serialize};

use crate::math::{Distance, Vector2, Vector3};
use std::f32::consts::PI;
use std::path::PathBuf;

use super::BodyStateMap;

#[derive(Clone, Copy, Default)]
pub struct Uniforms {
    model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
    pub light_color: Vec3,
    pub light_pos: Vec3,
}

#[repr(C)]
pub struct Vertex<T, U> {
    pos: T,
    normal: T,
    tex_coord: U,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Shape {
    #[default]
    Sphere,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub shape: Shape,
    pub texture: PathBuf,
    pub bodies: Vec<String>,

    #[serde(skip)]
    bindings: Option<Bindings>,
    #[serde(skip)]
    num_indices: usize,
}

impl Model {
    pub fn load(&mut self, context: &mut Context, root_path: &PathBuf) {
        let img_path = root_path.join(&self.texture);
        let img = ImageReader::open(img_path).unwrap().decode().unwrap();
        let texture_resource = Texture::from_data_and_format(
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
        let (vertices, indices) = generate_uv_sphere(20, 24);
        let geometry_vertex_buffer =
            Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &indices);

        self.bindings = Some(Bindings {
            vertex_buffers: vec![geometry_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture_resource],
        });
        self.num_indices = indices.len();
    }

    pub fn draw_bodies(
        &self,
        context: &mut Context,
        body_state_map: &BodyStateMap,
        uniforms: &Uniforms,
        scale: f32,
    ) {
        for body_label in &self.bodies {
            let body_state = body_state_map.get(body_label).unwrap();
            let inst_scale = body_state.diameter;
            let model_mat = Mat4::from_scale_rotation_translation(
                inst_scale * Vec3::ONE / scale,
                Quat::from_rotation_y(body_state.rot[1]),
                vec3(
                    body_state.pos.x() / scale,
                    body_state.pos.y() / scale,
                    body_state.pos.z() / scale,
                ),
            );
            let mut unif = uniforms.clone();
            unif.model = model_mat;

            context.apply_bindings(self.bindings.as_ref().unwrap());
            context.apply_uniforms(&unif);
            context.draw(0, self.num_indices as i32, 1);
        }
    }
}

/// Generates the vertices and indices of a UV sphere with the given
/// number of stacks and sectors, in local space, i.e. with coordinates
/// between -1.0 and 1.0 on all axes.
pub fn generate_uv_sphere(
    n_stacks: u32,
    n_sectors: u32,
) -> (Vec<Vertex<Vector3, Vector2>>, Vec<u32>) {
    let mut vertices = vec![];
    let mut indices = vec![];

    // First create bottom and top points
    vertices.push(Vertex {
        pos: Vector3::new(0.0, -1.0, 0.0),
        normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
        tex_coord: Vector2::new(0.5, 1.),
    });
    vertices.push(Vertex {
        pos: Vector3::new(0.0, 1.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0).normalize(),
        tex_coord: Vector2::new(0.5, 0.),
    });

    for stack_step in 1..n_stacks {
        let percent_stacks = stack_step as f32 / n_stacks as f32;
        let phi = -PI / 2.0 + PI * percent_stacks;

        // Create n_sectors+1. The first and last will have the same position coordinates
        // but allow for different texture coordinates.
        for sector_step in 0..=n_sectors {
            let percent_rotation = sector_step as f32 / n_sectors as f32;
            let theta = 2.0 * PI * percent_rotation;
            let z_proj_magnitude = phi.cos();
            let x = z_proj_magnitude * theta.cos();
            let y = phi.sin();
            let z = z_proj_magnitude * theta.sin();

            vertices.push(Vertex {
                pos: Vector3::new(x, y, z),
                normal: Vector3::new(x, y, z).normalize(),
                tex_coord: Vector2::new(1.0 - percent_rotation, 1.0 - percent_stacks),
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
