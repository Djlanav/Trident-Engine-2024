use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use gl::types::GLchar;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr;

#[derive(Clone)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct ShaderProgram {
    program_id: u32,
    shaders: Vec<Shader>,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> Self {
        let program_id = unsafe { gl::CreateProgram() };
        let shaders = shaders.to_vec();

        for shader in shaders.iter() {
            unsafe {
                gl::AttachShader(program_id, shader.shader_id);
            }
        }

        Self {
            program_id,
            shaders,
        }
    }

    pub fn link(&self) {
        let program_id = self.program_id;
        let mut link_status = 0;
        let mut log_length = 0;

        unsafe {
            gl::LinkProgram(program_id);
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut link_status);
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_length);

            if link_status != gl::TRUE as i32 {
                let mut written_len = 0;
                let mut buffer: Vec<u8> = Vec::with_capacity(log_length as usize);

                gl::GetProgramInfoLog(
                    program_id,
                    log_length,
                    &mut written_len,
                    buffer.as_mut_ptr() as *mut GLchar,
                );

                if let Ok(log) = CStr::from_ptr(buffer.as_ptr() as *const GLchar).to_str() {
                    println!("An error occurred in linking the shader program! ERROR: {}", log);
                }
            } else {
                println!("Shader program linked successfully");
            }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
}

#[derive(Clone)]
pub struct Shader {
    pub shader_type: ShaderType,
    pub shader_id: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}

impl Shader {
    fn compile_shader(shader_id: u32, shader_source: &String) {
        let src_c = CString::new(shader_source.as_bytes()).unwrap();

        unsafe {
            gl::ShaderSource(shader_id, 1, &src_c.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
        }

        let mut status = 0;
        let mut log_length = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_length);

            if status != 1 {
                let mut written_len = 0;
                let mut buffer: Vec<u8> = Vec::with_capacity(log_length as usize);

                gl::GetShaderInfoLog(
                    shader_id,
                    log_length,
                    &mut written_len,
                    buffer.as_mut_ptr() as *mut GLchar, );

                if let Ok(log) = CStr::from_ptr(buffer.as_ptr() as *const GLchar).to_str() {
                    println!("An error occurred in shader compilation: {}", log);
                }
            } else {
                println!("Shader compilation successful");
            }
        }
    }

    /// Load the contents of a GLSL file into memory
    /// and store them in a Rust string that can then be passed to a Shader's constructor.
    /// Returns a Result containing the constructed String with the contents.
    /// ---
    /// It is important to note that the shaders directory does **not** need to be present in the
    /// `source_path` string. That is appended automatically. Thus, the final string would be:
    /// `"shaders/some_dir/my_shader.glsl"` where a programmer need only provide
    /// `some_dir/my_shader.glsl`
    pub fn load_shader_source(source_path: &str) -> Result<String, Box<dyn Error>> {
        let formatted_path = format!("shaders/{}", source_path);
        let shader_path = Path::new(&formatted_path);
        let mut shader_file = File::open(shader_path)?;
        let mut source_string = String::new();

        shader_file.read_to_string(&mut source_string)?;

        Ok(source_string)
    }

    /// This constructs a new Shader object using the provided `shader_type` and `shader_source`
    /// arguments.
    pub fn new(shader_type: ShaderType, shader_source: String) -> Self {
        let mut shader_id = 0u32;

        match shader_type {
            ShaderType::Vertex => {
                unsafe {
                    shader_id = gl::CreateShader(VERTEX_SHADER);
                }

                if shader_id != 0 {
                    Self::compile_shader(shader_id, &shader_source);
                } else {
                    println!("gl::CreateShader failed for type VERTEX_SHADER!");
                }
            },
            ShaderType::Fragment => {
                unsafe {
                    shader_id = gl::CreateShader(FRAGMENT_SHADER);
                }

                if shader_id != 0 {
                    Self::compile_shader(shader_id, &shader_source);
                } else {
                    println!("gl::CreateShader failed for type FRAGMENT_SHADER!");
                }
            }
        }

        Self {
            shader_id,
            shader_type,
        }
    }
}