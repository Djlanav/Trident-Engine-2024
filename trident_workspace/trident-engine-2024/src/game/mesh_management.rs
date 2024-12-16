use std::fs::File;
use std::path::Path;
use crate::gl_loading::{BufferObject, BufferType, VertexArrayObject, VertexAttributePointer};

pub struct Mesh {
    pub name: String,
    vao: VertexArrayObject,
    vbo: BufferObject<f32>,
    ebo: BufferObject<u32>,
}

impl Mesh {
    pub fn new(name: &str, mesh_data: &[f32], texture_coords: &[f32]) -> Self {
        let mut data_vec = mesh_data.to_vec();
        let mut indices = Vec::with_capacity(mesh_data.len());

        data_vec.append(&mut texture_coords.to_vec());
        for index in 0..data_vec.len() {
            indices.push(index as u32);
        }

        let vao = VertexArrayObject::new(vec![
           VertexAttributePointer::new((0, 3, gl::FLOAT, gl::FALSE, 0, 0)),
           VertexAttributePointer::new((1, 2, gl::FLOAT, gl::FALSE, 0, 3 * size_of::<f32>())),
        ]);

        let vbo = BufferObject::new(data_vec, BufferType::ArrayBuffer);
        let ebo = BufferObject::new(indices, BufferType::ElementArrayBuffer);

        vao.bind();
        vbo.bind();
        ebo.bind();

        vao.set_attrib_pointer(0);
        vao.set_attrib_pointer(1);
        vao.enable_attrib_pointers();

        Self {
            name: name.to_string(),
            vao,
            vbo,
            ebo,
        }
    }
}