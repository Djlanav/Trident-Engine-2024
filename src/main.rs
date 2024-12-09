mod gl_loading;
mod shader_management;
mod shader_errors;

use std::error::Error;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLContext, Window};
use sdl2::{EventPump, Sdl, VideoSubsystem};

struct Application {
    sdl_context: Sdl,
    video: VideoSubsystem,
    window: Window,
    gl_context: GLContext,
    event_pump: EventPump,
}

impl Application {
    fn init() -> Result<(Sdl, VideoSubsystem, Window), Box<dyn Error>> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video
            .window("rust-sdl2", 800, 600)
            .position_centered()
            .opengl()
            .resizable()
            .build()?;

        Ok((sdl, video, window))
    }

    pub fn new() -> Result<Self, dyn Error> {
        let (sdl_context, video, window) = Self::init()
            .expect("Failed to init SDL");

        let gl_context = window.gl_create_context()?;
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            sdl_context,
            video,
            window,
            gl_context,
            event_pump,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut event_pump = &self.event_pump;
        let window = &self.window;

        'main: loop {
            for event in event_pump.poll_iter() {

            }

            window.gl_swap_window();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
