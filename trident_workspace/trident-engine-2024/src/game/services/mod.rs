use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use async_std::sync::RwLock;

pub mod input_actions;

type SendSyncService = dyn Service + Send + Sync;

pub struct ServiceManager {
    active_services: HashMap<String, Arc<RwLock<SendSyncService>>>
}

static SM_INSTANCE: LazyLock<Arc<RwLock<ServiceManager>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(ServiceManager::new()))
});

impl ServiceManager {
    pub fn new() -> ServiceManager {
        Self {
            active_services: HashMap::new()
        }
    }

    pub fn get_service_manager() -> Arc<RwLock<Self>> {
        SM_INSTANCE.clone()
    }

    pub fn add_service(&mut self, name: &str, service: Arc<RwLock<SendSyncService>>) {
        let name_string = name.to_string();
        self.active_services.insert(name_string, service);
    }

    pub fn get_service(&self, name: &str) -> Option<Arc<RwLock<SendSyncService>>> {
        match self.active_services.get(name) {
            Some(service) => Some(service.clone()),
            None => None
        }
    }
}

pub trait Service {}