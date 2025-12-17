use cacao::appkit;
use objc2_application_services;
use cidre::cf;
use crate::engine;

pub struct Controller {
    engine: engine::Engine,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            engine: engine::Engine::new(),
        }
    }

    pub fn start(&mut self) {
        // TODO
    }
    
    pub fn stop(&mut self) {
        // TODO
    }
}