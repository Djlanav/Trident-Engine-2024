use crate::game::services::Service;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct ActionsService {
    actions: HashMap<Keycode, Vec<Action>>,
}

impl Service for ActionsService {}

impl ActionsService {
    pub fn new() -> ActionsService {
        Self {
            actions: HashMap::new(),
        }
    }
    
    pub fn register_action(&mut self, action: Action) {
        self.actions
            .entry(action.assigned_key)
            .or_insert_with(Vec::new)
            .push(action);
    }

    pub async fn fire_event(&self, key: Keycode) {
        if let Some(actions) = self.actions.get(&key) {
            for action in actions {
                action.call().await;
            }
        }
    }
}

pub enum CallbackType {
    Sync,
    Async,
}

enum ActionCallback {
    Sync(Arc<dyn Fn() + Send + Sync>),
    Async(Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>),
}

pub struct Action {
    pub name: String,
    pub assigned_key: Keycode,
    pub description: Option<String>,
    callback: ActionCallback,
    callback_type: CallbackType,
}

impl Action {
    pub fn new<F>(
        name: String, 
        assigned_key: Keycode, 
        description: Option<String>,
        callback: F) -> Action 
    where F: Fn() + Send + Sync + 'static
    {
        Self {
            name,
            assigned_key,
            description,
            callback: ActionCallback::Sync(Arc::new(callback)),
            callback_type: CallbackType::Sync,
        }
    }

    pub fn new_async<F, Fut>(
        name: String,
        assigned_key: Keycode,
        description: Option<String>,
        callback: F) -> Action
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static
    {
        Self {
            name,
            assigned_key,
            description,
            callback: ActionCallback::Async(Arc::new(move || {
                Box::pin(callback()) as Pin<Box<dyn Future<Output = ()> + Send>>
            })),
            callback_type: CallbackType::Async,
        }
    }

    pub fn check_callback_type(&self) -> &CallbackType {
        &self.callback_type
    }
    
    pub async fn call(&self) {
        match &self.callback {
            ActionCallback::Sync(callback) => callback(),
            ActionCallback::Async(callback) => callback().await
        }
    }
}