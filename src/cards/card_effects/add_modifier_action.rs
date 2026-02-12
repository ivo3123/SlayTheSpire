use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};
use crate::core::base_state::{Modifier, State};

#[derive(Clone, Debug)]
pub struct AddModifierAction {
    pub modifier: Modifier,
}

impl Action for AddModifierAction {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _targets: &[EntityId], _energy_spent: Option<i32>) {
        match source {
            EntityId::Player => game_state.player_mut().add_modifier(self.modifier.clone()),
            EntityId::Enemy(id) => {
                if let Some(enemy) = game_state.enemies_mut().get_mut(id) {
                    enemy.add_modifier(self.modifier.clone());
                }
            }
        }
    }
    
    fn description(&self) -> String {
        match self.modifier {
            Modifier::RetainBlock => "Block is not removed at the start of your turn".to_string(),
            Modifier::RetainHand => "Do not discard hand at end of turn".to_string(),
        }
    }
    
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
