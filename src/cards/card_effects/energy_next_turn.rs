use crate::core::effects::{Effect, EffectUIState};
use crate::core::game_state::{GameState, GameEvent, EntityId};

#[derive(Clone, Debug)]
pub struct EnergyNextTurnEffect {
    pub amount: i32,
    activated: bool,
}

impl EnergyNextTurnEffect {
    pub fn new(amount: i32) -> Self {
        EnergyNextTurnEffect {
            amount,
            activated: false,
        }
    }
}

impl Effect for EnergyNextTurnEffect {
    fn on_event(&mut self, event: &GameEvent, owner: EntityId, game_state: &mut GameState) {
        if let GameEvent::TurnStarted { entity } = event {
            if *entity == owner {
                if let EntityId::Player = owner {
                    game_state.player_mut().gain_energy(self.amount);
                    self.activated = true;
                }
            }
        }
    }
    
    fn should_remove(&self) -> bool {
        self.activated
    }
    
    fn ui_state(&self) -> EffectUIState {
        EffectUIState {
            name: format!("Energy Next Turn"),
            description: format!("At the start of next turn, gain {} Energy.", self.amount),
            counters: vec![],
        }
    }
    
    fn clone_box(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}
