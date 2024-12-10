mod gl_loading;
mod shader_management;
mod shader_errors;
mod application;
mod opengl_utils;

use std::error::Error;
use crate::application::Application;
use crate::gl_loading::{VertexArrayObject, VertexAttributePointer, VertexBufferObject};
use crate::shader_management::{Shader, ShaderProgram, ShaderType};

fn make_shader_stuff() -> ShaderProgram{
    let vertex_source = Shader::load_shader_source("main_vertex.glsl")
        .expect("Could not load vertex shade source");
    let fragment_source = Shader::load_shader_source("main_fragment.glsl")
        .expect("Could not load fragment shader source");

    let vertex_shader = Shader::new(ShaderType::Vertex, vertex_source);
    let fragment_shader = Shader::new(ShaderType::Fragment, fragment_source);

    let shaders = [vertex_shader, fragment_shader];
    let shader_program = ShaderProgram::new(&shaders);
    shader_program.link();

    shader_program
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut application = Application::new().expect("Failed to init SDL");
    let vertices = [
        -0.5f32, 0.5, 0.0,
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.5, 0.5, 0.0,
    ];

    let attrib_vec: Vec<VertexAttributePointer> = Vec::new();

    let mut vao = VertexArrayObject::new(attrib_vec);
    vao.bind();

    let vbo = VertexBufferObject::new(&vertices);
    vbo.bind();

    let attrib_ptr = VertexAttributePointer::new((
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        (3 * size_of::<f32>()) as gl::types::GLsizei,
        0
    ));

    vao.add_attrib_pointer(attrib_ptr);
    vao.enable_attrib_pointers();

    let shader_program = make_shader_stuff();
    shader_program.use_program();

    application.run(|| {


        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as gl::types::GLsizei);
        }

    }).expect("Failed to run SDL application");

    Ok(())
}
