use std::ffi::CString;
use std::ptr;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    shader_type: ShaderType,
    shader_id: u32,
}

impl Shader {
    fn compile_shader(shader_id: u32, shader_source: &String) {
        let src_c = CString::new(shader_source.as_bytes()).unwrap();

        unsafe {
            gl::ShaderSource(shader_id, 1, &src_c.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
        }
    }

    pub fn new(shader_type: ShaderType, shader_source: String) -> Self {
        let mut shader_id = 0u32;

        match shader_type {
            ShaderType::Vertex => {
                unsafe {
                    shader_id = gl::CreateShader(VERTEX_SHADER);
                }
                Self::compile_shader(shader_id, &shader_source);
            },
            ShaderType::Fragment => {
                unsafe {
                    shader_id = gl::CreateShader(FRAGMENT_SHADER);
                }
                Self::compile_shader(shader_id, &shader_source);
            }
        }

        Self {
            shader_id,
            shader_type,
        }
    }
}