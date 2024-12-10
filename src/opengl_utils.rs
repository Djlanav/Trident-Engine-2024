use gl::types::GLenum;
use log::{error, info, warn};

pub fn check_opengl_error() {
    unsafe {
        let gl_error = gl::GetError();

        match gl_error {
            gl::NO_ERROR => info!("OpenGL Status: No error"),
            gl::INVALID_ENUM => error!("OpenGL Status: Invalid OpenGL enum value"),
            gl::INVALID_VALUE => error!("OpenGL Status: Invalid OpenGL value"),
            gl::INVALID_OPERATION => error!("OpenGL Status: Invalid OpenGL operation"),
            gl::INVALID_FRAMEBUFFER_OPERATION => error!("OpenGL Status: Invalid OpenGL framebuffer operation"),
            gl::OUT_OF_MEMORY => error!("OpenGL Status: Out of OpenGL memory"),
            gl::STACK_UNDERFLOW => error!("OpenGL stack underflow"),
            gl::STACK_OVERFLOW => error!("OpenGL stack overflow"),
            _ => panic!("OpenGL error: {}", gl_error)
        }
    }
}