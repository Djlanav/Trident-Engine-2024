use async_std::fs::File;
use async_std::io::ReadExt;
use async_std::task;
use std::path::Path;
use crate::gl_loading::{BufferObject, BufferType, VertexArrayObject, VertexAttributePointer};

pub struct Mesh {
    pub name: String,
    vao: VertexArrayObject,
    vbo: BufferObject<f32>,
    ebo: BufferObject<u32>,
}

impl Mesh {
    pub fn new(name: &str, mut mesh_data: Vec<f32>, texture_coords: Vec<f32>) -> Self {
        let mut indices = Vec::with_capacity(mesh_data.len());

        mesh_data.append(&mut texture_coords.to_vec());
        for index in 0..mesh_data.len() {
            indices.push(index as u32);
        }

        let vao = VertexArrayObject::new(vec![
           VertexAttributePointer::new((0, 3, gl::FLOAT, gl::FALSE, 0, 0)),
           VertexAttributePointer::new((1, 2, gl::FLOAT, gl::FALSE, 0, 3 * size_of::<f32>())),
        ]);

        let vbo = BufferObject::new(mesh_data, BufferType::ArrayBuffer);
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

    pub async fn from_file(mesh_name: &str, path: &str) -> Option<Self> {
        let path = Path::new(path);

        if let Ok(mut file) = File::open(path).await {
            let mut contents = String::new();
            file.read_to_string(&mut contents).await.unwrap();

            let contents = contents
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let contents_task = task::spawn(async move {
                let mut mesh_data = Vec::new();
                let mut texture_coords = Vec::new();

                for line in contents.chunks(4) {
                    if line[0] == "tc" {
                        break;
                    }

                    let vertex_pos = &line[1..];
                    for point in vertex_pos {
                        let point = point.parse::<f32>().unwrap();
                        mesh_data.push(point);
                    }
                }

                for coord in contents.chunks(3) {
                    let coords = &coord[1..];
                    for point in coords {
                        let point = point.parse::<f32>().unwrap();
                        texture_coords.push(point);
                    }
                }

                (mesh_data, texture_coords)
            });

            let (mesh_data, texture_coords) = contents_task.await;
            let mesh = Self::new(mesh_name, mesh_data, texture_coords);
            Some(mesh)
        } else {
            None
        }
    }

    pub fn get_ebo(&self) -> &BufferObject<u32> {
        &self.ebo
    }
}