use std::collections::HashMap;
use std::ffi::c_void;
use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use crate::opengl_utils::check_opengl_error;

pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormatEnum,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

pub struct TextureLoader {
    texture_map: HashMap<String, Texture>
}

impl TextureLoader {
    pub fn new() -> Self {
        Self {
            texture_map: HashMap::new()
        }
    }

    fn get_texture_data(&self, texture_path: &str) -> (Vec<u8>, u32, u32) {
        let surface = Surface::from_file(texture_path).unwrap();
        let surface = surface.convert_format(PixelFormatEnum::RGBA32).unwrap();
        let (width, height) = surface.size();

        let pixel_data = surface.without_lock().unwrap();
        (pixel_data.to_vec(), width, height)
    }

    pub fn load_texture(&mut self, texture_path: &str) {
        let (pixel_data, width, height) = self.get_texture_data(texture_path);

        let mut texture_id = 0u32;
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixel_data.as_ptr() as *const c_void,);

            gl::GenerateMipmap(gl::TEXTURE_2D);
            #[cfg(debug_assertions)]
            check_opengl_error("texture_management", 67);
        }

        let texture = Texture {
            id: texture_id,
            width,
            height,
            format: PixelFormatEnum::RGBA32,
        };

        let texture_name = texture_path.split("/").last().unwrap().to_string();
        self.texture_map.insert(texture_name, texture);
    }
}