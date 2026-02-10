use crate::core::game_state::{GameState, GameEvent, EntityId};

pub struct EffectUIState {
    pub name: String,
    pub description: String,
    pub counters: Vec<(String, i32)>,
}

pub trait Effect: std::fmt::Debug {
    fn on_event(&mut self, event: &GameEvent, owner: EntityId, game_state: &mut GameState);
    fn ui_state(&self) -> EffectUIState;
    fn should_remove(&self) -> bool { false }
    fn clone_box(&self) -> Box<dyn Effect>;
}
