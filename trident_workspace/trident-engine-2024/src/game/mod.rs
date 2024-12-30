use mesh_management::Mesh;

pub mod mesh_management;
pub mod services;

pub trait GameObject {
    fn get_mesh(&self) -> &Mesh;
    fn update(&mut self);
}