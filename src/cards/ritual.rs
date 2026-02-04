use crate::core::effects::{Effect, EffectUIState, GameContext};
use crate::core::game_state::{GameEvent, EntityId};
use crate::core::base_state::StatusType;

/// Ritual - Gain strength at end of turn
#[derive(Debug)]
pub struct Ritual {
    pub amount: i32,
}

impl Effect for Ritual {
    fn on_event(&mut self, event: &GameEvent, ctx: &mut GameContext) {
        if let GameEvent::TurnEnded { entity } = event {
            if *entity == ctx.owner {
                ctx.add_status(ctx.owner, StatusType::Strength, self.amount);
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
}
