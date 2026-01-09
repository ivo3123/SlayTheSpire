use crate::mechanics::base_state::BaseState;

#[derive(Clone, Debug)]
pub enum EnemyIntent {
    Attack(i32),
    Defend(i32),
    GainStrength(i32),
    ApplyWeak(i32),
    MultiAttack(i32, i32),   // damage_per_hit, num_hits
    Unkown,
}

#[derive(Clone, Debug)]
pub struct BaseEnemyState {
    base_state: BaseState,
    id: String,
    next_move: EnemyIntent,
}

impl BaseEnemyState {
    pub fn new(id: String, name: String, max_health: i32) -> Self {
        BaseEnemyState {
            base_state: BaseState::new(name, max_health),
            id,
            next_move: EnemyIntent::Unkown,
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn next_move(&self) -> &EnemyIntent {
        &self.next_move
    }
    
    pub fn base_state(&self) -> &BaseState {
        &self.base_state
    }

    pub fn base_state_mut(&mut self) -> &mut BaseState {
        &mut self.base_state
    }
}

