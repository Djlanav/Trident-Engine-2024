mod gl_loading;
mod shader_management;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // OpenGL Attribs
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context()?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let mut event_pump = sdl_context.event_pump()?;
    unsafe {
        gl::ClearColor(0.0, 0.6, 0.8, 1.0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'main; }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }

    Ok(())
}
