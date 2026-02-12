use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};

#[derive(Clone, Debug)]
pub struct DamageEffect {
    pub amount: i32,
}

impl Action for DamageEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, targets: &[EntityId], energy_spent: Option<i32>) {
        let damage = energy_spent.unwrap_or(self.amount);
        
        for &target_id in targets {
            game_state.deal_damage(source, target_id, damage);
        }
    }
    
    fn description(&self) -> String {
        format!("Deal {} damage", self.amount)
    }
    
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
