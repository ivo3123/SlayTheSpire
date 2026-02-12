use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};
use crate::core::effects::Effect;

#[derive(Clone)]
pub struct ApplyEffect {
    pub effect: Box<dyn Effect>,
}

impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for ApplyEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplyEffect").finish()
    }
}

impl Action for ApplyEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _targets: &[EntityId], _energy_spent: Option<i32>) {
        game_state.add_effect(source, self.effect.clone_box());
    }
    
    fn description(&self) -> String {
        self.effect.ui_state().description
    }
    
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
