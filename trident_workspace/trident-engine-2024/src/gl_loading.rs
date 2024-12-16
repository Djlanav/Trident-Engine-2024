use gl::types::{GLboolean, GLenum, GLsizei};
use std::any::Any;
use std::ops::Deref;
use std::os::raw::c_void;
use log::error;
use crate::opengl_utils::check_opengl_error;

pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

pub struct BufferObject<T>
where
    T: Sized + 'static + Clone
{
    id: u32,
    buffer_type: BufferType,
    data: Vec<T>,
}

impl<T: 'static + Clone> BufferObject<T> {
    pub fn new<B>(data: B, buffer_type: BufferType) -> Self
    where
        B: Into<Vec<T>> + Deref<Target=[T]>,
    {
        assert!(!data.is_empty());
        let mut id = 0;

        unsafe {
            match buffer_type {
                BufferType::ArrayBuffer => {
                    gl::GenBuffers(1, &mut id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, id);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        size_of_val(&data) as isize,
                        data.as_ptr() as *const c_void,
                        gl::STATIC_DRAW);
                },
                BufferType::ElementArrayBuffer => {
                    gl::GenBuffers(1, &mut id);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);

                    let any_value = &data[0] as &dyn Any;
                    if any_value.is::<f32>() {
                        panic!("Element array buffer cannot contain f32 values");
                    }

                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        size_of_val(&data) as isize,
                        data.as_ptr() as *const c_void,
                        gl::STATIC_DRAW);
                },
            }

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 55);
        }

        Self {id, data: data.to_vec(), buffer_type}
    }

    pub fn bind(&self) {
        unsafe {
            match self.buffer_type {
                BufferType::ArrayBuffer => gl::BindBuffer(gl::ARRAY_BUFFER, self.id),
                BufferType::ElementArrayBuffer => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id),
            }

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 69);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            match self.buffer_type {
                BufferType::ArrayBuffer => gl::BindBuffer(gl::ARRAY_BUFFER, 0),
                BufferType::ElementArrayBuffer => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0),
            }

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 81);
        }
    }
}

impl<T> Drop for BufferObject<T>
where
    T: Sized + 'static + Clone
{
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteBuffers(1, &self.id);

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 94);
        }
    }
}

pub struct VertexArrayObject {
    id: u32,
    attrib_pointers: Vec<VertexAttributePointer>
}

impl VertexArrayObject {
    pub fn new(attrib_pointers: Vec<VertexAttributePointer>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 111);
        }

        Self {
            id, attrib_pointers
        }
    }

    pub fn set_attrib_pointer(&self, index: usize) {
        let pointer =  match self.attrib_pointers.get(index) {
            Some(pointer) => pointer,
            None => {
                error!("No attrib pointer at index {}", index);
                return;
            },
        };

        pointer.set_attrib_pointer();
    }

    pub fn enable_attrib_pointers(&self) {
        for vap in self.attrib_pointers.iter() {
            vap.enable_vertex_attrib_ptr();
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 142);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 151);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteVertexArrays(1, &self.id);

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 164);
        }
    }
}

pub struct VertexAttributePointer {
    index: u32,
    size: i32,
    data_type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    offset: usize,
}

impl VertexAttributePointer {
    pub fn new(data_tuple: (u32, i32, GLenum, GLboolean, usize, usize)) -> Self {
        let (index,
            size,
            data_type,
            normalized,
            _stride,
            offset) = data_tuple;

        Self {
            index,
            size,
            data_type,
            normalized,
            stride: data_tuple.4 as GLsizei,
            offset,
        }
    }

    pub fn set_attrib_pointer(&self) {
        unsafe {
            gl::VertexAttribPointer(
                self.index,
                self.size,
                self.data_type,
                self.normalized,
                self.stride,
                self.offset as *const c_void);
        }
    }

    pub fn enable_vertex_attrib_ptr(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);

            #[cfg(debug_assertions)]
            check_opengl_error("gl_loading", 214);
        }
    }
}