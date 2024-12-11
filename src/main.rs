mod gl_loading;
mod shader_management;
mod shader_errors;
mod application;
mod opengl_utils;
mod engine_types;
mod macros;

use std::error::Error;
use crate::application::Application;
use crate::engine_types::{Vector3, Vector4};
use crate::gl_loading::{VertexArrayObject, VertexAttributePointer, BufferObject, BufferType};
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
        -0.5f32, 0.5, 0.0, // 0
        -0.5, -0.5, 0.0, // 1
        0.5, -0.5, 0.0, // 2
        0.5, 0.5, 0.0, // 3
    ];

    let indices = [
        0u32, 1, 2,
        2, 3, 0,
    ];

    let attrib_vec: Vec<VertexAttributePointer> = Vec::new();

    let mut vao = VertexArrayObject::new(attrib_vec);
    vao.bind();

    let vbo = BufferObject::new(&vertices, BufferType::ArrayBuffer);
    let ebo = BufferObject::new(&indices, BufferType::ElementArrayBuffer);
    vbo.bind();
    ebo.bind();

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

    let mut shader_program = make_shader_stuff();

    let my_vector = Vector4::new(1.0, 0.0, 0.0, 0.0);
    shader_program.get_uniform_locations(&["u_Color"]);
    shader_program.use_program();

    application.run(|| {
        shader_program.set_uniform_vec4("u_Color", &my_vector);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null())
        }

    }).expect("Failed to run SDL application");

    Ok(())
}
