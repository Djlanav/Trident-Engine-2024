use std::error::Error;
use log::warn;
use trident_engine_2024::game::GameObject;
use trident_engine_2024::game::mesh_management::Mesh;
use trident_engine_2024::game::services::ServiceManager;

pub struct DefaultObject {
    name: String,
    mesh: Option<Mesh>,
}

impl DefaultObject {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mesh: Mesh::from_file("My Mesh", "res/models/my_mesh.txt").await,
        }
    }
}

impl GameObject for DefaultObject {
    fn get_mesh(&self) -> &Mesh {
        self.mesh.as_ref().unwrap()
    }

    fn update(&mut self) {
        warn!("Player update method not implemented yet.");
    }
}

pub async fn init() {
    let sm = ServiceManager::get_service_manager();
    let sm_read = sm.read().await;
    
    let actions = sm_read
        .get_service("ActionsService")
        .unwrap();
    let actions_read = actions.read().await;
    
    let my_object = DefaultObject::new("PlayerObject");
}

pub fn play() -> Result<(), Box<dyn Error>> {
    Ok(())
}