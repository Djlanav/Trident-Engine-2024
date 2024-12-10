use std::any::{Any, TypeId};
use std::os::raw::c_void;
use gl::types::{GLboolean, GLenum, GLsizei};
use crate::opengl_utils;

pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

pub struct BufferObject<'buffer_lifetime, T>
where
    T: Sized + 'static
{
    id: u32,
    buffer_type: BufferType,
    data: &'buffer_lifetime [T],
}

impl<'buffer_lifetime, T: 'static> BufferObject<'buffer_lifetime, T> {
    pub fn new(data: &'buffer_lifetime [T], buffer_type: BufferType) -> Self {
        assert!(!data.is_empty());
        let mut id = 0;

        unsafe {
            match buffer_type {
                BufferType::ArrayBuffer => {
                    gl::GenBuffers(1, &mut id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, id);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        size_of_val(data) as isize,
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
                        size_of_val(data) as isize,
                        data.as_ptr() as *const c_void,
                        gl::STATIC_DRAW);
                },
            }

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }

        Self {id, data, buffer_type}
    }

    pub fn bind(&self) {
        unsafe {
            match self.buffer_type {
                BufferType::ArrayBuffer => gl::BindBuffer(gl::ARRAY_BUFFER, self.id),
                BufferType::ElementArrayBuffer => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id),
            }

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }
    }

    pub fn unbind(&self) {
        unsafe {
            match self.buffer_type {
                BufferType::ArrayBuffer => gl::BindBuffer(gl::ARRAY_BUFFER, 0),
                BufferType::ElementArrayBuffer => gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0),
            }

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }
    }
}

impl<T: 'static> Drop for BufferObject<'_, T> {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteBuffers(1, &self.id);

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
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
            opengl_utils::check_opengl_error();
        }

        Self {
            id, attrib_pointers
        }
    }

    pub fn add_attrib_pointer(&mut self, attrib_pointer: VertexAttributePointer) {
        self.attrib_pointers.push(attrib_pointer);
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
            opengl_utils::check_opengl_error();
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteVertexArrays(1, &self.id);

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }
    }
}

pub struct VertexAttributePointer {
    index: u32,
    size: i32,
    data_type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    offset: u32,
}

impl VertexAttributePointer {
    pub fn new(data_tuple: (u32, i32, GLenum, GLboolean, GLsizei, u32)) -> Self {
        let (index,
            size,
            data_type,
            normalized,
            stride,
            offset) = data_tuple;

        unsafe {
            // TODO: Fix invalid OpenGL operation error caused by this call
            gl::VertexAttribPointer(index, size, data_type, normalized, stride, offset as *const c_void);

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }

        Self {
            index,
            size,
            data_type,
            normalized,
            stride,
            offset,
        }
    }

    pub fn enable_vertex_attrib_ptr(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);

            #[cfg(debug_assertions)]
            opengl_utils::check_opengl_error();
        }
    }
}