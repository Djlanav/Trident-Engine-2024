use std::fs::File;
use std::path::Path;
use crate::gl_loading::{VertexArrayObject, VertexAttributePointer};

pub struct Mesh {
    vao: VertexArrayObject,
    name: String,
}

impl Mesh {
    pub fn new(model_path: &str) {
        let model_path = Path::new(model_path);
        let model_file = File::open(model_path).expect("Failed to open model file");
    }
}