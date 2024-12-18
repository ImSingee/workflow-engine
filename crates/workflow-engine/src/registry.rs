// use crate::action::{ActionBridge, ActionBridgeTrait};
use crate::Action;

pub struct Registry {
    // actions: Vec<Box<dyn ActionBridgeTrait>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn register_action(&mut self, action: impl Action) {
        // TODO
        // self.actions.pu
    }
}
