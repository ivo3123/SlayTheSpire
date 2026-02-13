use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};

#[derive(Debug, Clone)]
pub struct XDamageEffect {
    pub damage_per_energy: i32,
}

impl Action for XDamageEffect {
    fn resolve(
        &self,
        game_state: &mut GameState,
        source: EntityId,
        targets: &[EntityId],
        energy_spent: Option<i32>,
    ) {
        let x = energy_spent.unwrap_or(0);
        let total_damage = self.damage_per_energy * x;
        
        for &target in targets {
            game_state.deal_damage(source, target, total_damage);
        }
    }
    
    fn description(&self) -> String {
        format!("Deal {} damage X times", self.damage_per_energy)
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
