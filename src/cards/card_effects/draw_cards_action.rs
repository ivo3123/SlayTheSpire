use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};

#[derive(Debug, Clone)]
pub struct DrawCardsAction {
    pub count: usize,
}

impl Action for DrawCardsAction {
    fn resolve(
        &self,
        game_state: &mut GameState,
        _source: EntityId,
        _targets: &[EntityId],
        _energy_spent: Option<i32>,
    ) {
        game_state.draw_cards(self.count);
    }
    
    fn description(&self) -> String {
        format!("Draw {} card{}", self.count, if self.count == 1 { "" } else { "s" })
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
