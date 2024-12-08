use std::os::raw::c_void;

pub struct VertexBufferObject<'buffer_lifetime> {
    id: u32,
    data: &'buffer_lifetime [f32]
}

impl<'buffer_lifetime> VertexBufferObject<'buffer_lifetime> {
    pub fn new(data: &'buffer_lifetime [f32]) -> Self {
        assert!(!data.is_empty());
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(data) as isize,
                data.as_ptr() as *const c_void,
                gl::STATIC_DRAW);
        }

        Self {id, data}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBufferObject<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self {id}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}