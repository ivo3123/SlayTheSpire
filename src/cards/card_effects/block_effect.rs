use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};

#[derive(Clone, Debug)]
pub struct BlockEffect {
    pub amount: i32,
}

impl Action for BlockEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _targets: &[EntityId], energy_spent: Option<i32>) {
        let block = energy_spent.unwrap_or(self.amount);
        game_state.gain_block(source, block);
    }
    
    fn description(&self) -> String {
        format!("Gain {} Block", self.amount)
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
