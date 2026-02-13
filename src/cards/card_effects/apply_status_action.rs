use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};
use crate::core::base_state::{StatusType, State};

#[derive(Debug, Clone)]
pub struct ApplyStatusAction {
    pub status_type: StatusType,
    pub stacks: i32,
}

impl Action for ApplyStatusAction {
    fn resolve(
        &self,
        game_state: &mut GameState,
        _source: EntityId,
        targets: &[EntityId],
        _energy_spent: Option<i32>,
    ) {
        for &target in targets {
            match target {
                EntityId::Player => {
                    game_state.player_mut().add_status(self.status_type.clone(), self.stacks);
                }
                EntityId::Enemy(id) => {
                    if let Some(enemy) = game_state.enemies_mut().get_mut(id) {
                        enemy.add_status(self.status_type.clone(), self.stacks);
                    }
                }
            }
        }
    }
    
    fn description(&self) -> String {
        format!("Apply {} {:?}", self.stacks, self.status_type)
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
