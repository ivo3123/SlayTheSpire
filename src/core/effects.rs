use crate::core::game_state::{GameState, GameEvent, EntityId};
use crate::core::base_state::{StatusType, State};

pub struct GameContext<'a> {
    pub owner: EntityId,
    pub game_state: &'a mut GameState,
}

impl<'a> GameContext<'a> {
    pub fn add_status(&mut self, target: EntityId, status: StatusType, stacks: i32) {
        match target {
            EntityId::Player => self.game_state.player.add_status(status, stacks),
            EntityId::Enemy(id) => {
                if let Some(enemy) = self.game_state.enemies.get_mut(id) {
                    enemy.add_status(status, stacks);
                }
            }
        }
    }
}

pub struct EffectUIState {
    pub name: String,
    pub description: String,
    pub counters: Vec<(String, i32)>,
}

pub trait Effect: std::fmt::Debug {
    fn on_event(&mut self, event: &GameEvent, ctx: &mut GameContext);
    fn ui_state(&self) -> EffectUIState;
    fn should_remove(&self) -> bool { false }
    fn clone_box(&self) -> Box<dyn Effect>;
}
