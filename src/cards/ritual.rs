use crate::core::effects::{Effect, EffectUIState};
use crate::core::game_state::{GameEvent, GameState, EntityId};
use crate::core::base_state::{StatusType, State};

#[derive(Debug, Clone)]
pub struct Ritual {
    pub amount: i32,
}

impl Effect for Ritual {
    fn on_event(&mut self, event: &GameEvent, owner: EntityId, game_state: &mut GameState) {
        if let GameEvent::TurnEnded { entity } = event {
            if *entity == owner {
                match owner {
                    EntityId::Player => game_state.player_mut().add_status(StatusType::Strength, self.amount),
                    EntityId::Enemy(id) => {
                        if let Some(enemy) = game_state.enemies_mut().get_mut(id) {
                            enemy.add_status(StatusType::Strength, self.amount);
                        }
                    }
                }
            }
        }
    }

    fn ui_state(&self) -> EffectUIState {
        EffectUIState {
            name: "Ritual".into(),
            description: format!("At end of turn gain {} Strength", self.amount),
            counters: vec![],
        }
    }
    
    fn clone_box(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}
