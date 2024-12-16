use log::{error, info};

pub fn check_opengl_error(file_name: &str, line_number: u32) {
    unsafe {
        let gl_error = gl::GetError();

        match gl_error {
            gl::INVALID_ENUM => error!("OpenGL Status: Invalid OpenGL enum value at line {} in file {}", line_number, file_name),
            gl::INVALID_VALUE => error!("OpenGL Status: Invalid OpenGL value at line {} in file {}", line_number, file_name),
            gl::INVALID_OPERATION => error!("OpenGL Status: Invalid OpenGL operation at line {} in file {}", line_number, file_name),
            gl::INVALID_FRAMEBUFFER_OPERATION => error!("OpenGL Status: Invalid OpenGL framebuffer operation at line {} in file {}", line_number, file_name),
            gl::OUT_OF_MEMORY => error!("OpenGL Status: Out of OpenGL memory at line {} in file {}", line_number, file_name),
            gl::STACK_UNDERFLOW => error!("OpenGL stack underflow at line {} in file {}", line_number, file_name),
            gl::STACK_OVERFLOW => error!("OpenGL stack overflow at line {} in file {}", line_number, file_name),
            _ => {},
        }
    }
}