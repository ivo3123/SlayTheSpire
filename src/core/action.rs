use crate::core::game_state::{GameState, EntityId};

pub trait Action: std::fmt::Debug {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, targets: &[EntityId], energy_spent: Option<i32>);
    fn description(&self) -> String;
    fn clone_box(&self) -> Box<dyn Action>;
}

#[derive(Clone)]
pub struct Intent {
    actions: Vec<Box<dyn Action>>,
    description: String,
}

impl Intent {
    pub fn new(actions: Vec<Box<dyn Action>>, description: String) -> Self {
        Intent { actions, description }
    }
    
    pub fn description(&self) -> &str {
        &self.description
    }
    
    pub fn execute(&self, game_state: &mut GameState, source: EntityId, targets: &[EntityId], energy_spent: Option<i32>) {
        for action in &self.actions {
            action.resolve(game_state, source, targets, energy_spent);
        }
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
