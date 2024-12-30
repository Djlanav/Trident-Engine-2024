use crate::game::mesh_management::Mesh;

pub fn prepare_rendering() {
    unsafe {
        gl::ClearColor(0.0, 0.5, 0.7, 1.0);
    }
}

pub fn render(mesh: &Mesh) {

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.get_ebo().get_data_len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null());
    }
}