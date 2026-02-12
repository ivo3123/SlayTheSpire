use crate::core::effects::{Effect, EffectUIState};
use crate::core::game_state::{GameState, GameEvent, EntityId};
use crate::core::base_state::{StatusType, State};

#[derive(Clone, Debug)]
pub struct Ritual {
    pub amount: i32,
}

impl Effect for Ritual {
    fn on_event(&mut self, event: &GameEvent, owner: EntityId, game_state: &mut GameState) {
        if let GameEvent::TurnEnded { entity } = event {
            if *entity == owner {
                if let EntityId::Player = owner {
                    game_state.player_mut().add_status(StatusType::Strength, self.amount);
                }
            }
        }
    }
    
    fn ui_state(&self) -> EffectUIState {
        EffectUIState {
            name: format!("Ritual"),
            description: format!("At the end of your turn, gain {} Strength.", self.amount),
            counters: vec![],
        }
    }
    
    fn clone_box(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}
