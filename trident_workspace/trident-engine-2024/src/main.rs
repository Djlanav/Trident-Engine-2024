mod gl_loading;
mod shader_management;
mod shader_errors;
mod application;
mod opengl_utils;
mod engine_types;
mod macros;
mod client;

use std::error::Error;
use std::ops::Add;
use std::thread;
use std::time::{Duration, Instant};
use log::info;
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
        -0.5f32, 0.5, 0.0, 1.0, 0.0, 0.0, // Vertex 0
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // Vertex 1
        0.5, -0.5, 0.0, 0.0, 0.0, 1.0, // Vertex 2
        0.5, 0.5, 0.0, 0.0, 0.0, 0.0 // Vertex 3
    ];

    let indices = [
        0u32, 1, 2,
        2, 3, 0,
    ];

    let mut vao = VertexArrayObject::new(vec![
        VertexAttributePointer::new((0, 3, gl::FLOAT, gl::FALSE, 6 * size_of::<f32>(), 0)),
        VertexAttributePointer::new((1, 3, gl::FLOAT, gl::FALSE, 6 * size_of::<f32>(), 3 * size_of::<f32>())),
    ]);

    let vbo = BufferObject::new(&vertices, BufferType::ArrayBuffer);
    let ebo = BufferObject::new(&indices, BufferType::ElementArrayBuffer);

    vao.bind();
    vbo.bind();
    ebo.bind();

    vao.set_attrib_pointer(0);
    vao.set_attrib_pointer(1);
    vao.enable_attrib_pointers();

    let mut shader_program = make_shader_stuff();

    let my_vector = Vector3::new(0.5, 0.0, 0.7);
    shader_program.get_uniform_locations(&["u_Color"]);
    shader_program.use_program();

    let target_fps = 60;
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);

    let mut last_frame_time = Instant::now();

    application.run(|window| {
        let frame_start = Instant::now();
        shader_program.set_uniform_vec3("u_Color", &my_vector);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null())
        }

        let elapsed_time = frame_start.elapsed();
        if elapsed_time < frame_duration {
            thread::sleep(frame_duration - elapsed_time);
        }

        let delta_time = last_frame_time.elapsed();
        window.set_title(&format!("Trident Engine - OpenGL | Delta time: {:.2?}", delta_time))
            .expect("TODO: panic message");
        last_frame_time = Instant::now();

    }).expect("Failed to run SDL application");

    Ok(())
}
