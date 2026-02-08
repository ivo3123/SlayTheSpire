use crate::core::enemy::BaseEnemy;
use crate::core::base_state::State;

/// Simple enemy that attacks for 8 damage every turn
pub struct Dragonling {
    base: BaseEnemy,
}

impl Dragonling {
    pub fn new() -> Self {
        Dragonling {
            base: BaseEnemy::new("dragonling".to_string(), "Dragonling".to_string(), 50),
        }
    }
}

impl State for Dragonling {
    fn get_name(&self) -> &str {
        self.base.get_name()
    }
    
    fn get_max_health(&self) -> i32 {
        self.base.get_max_health()
    }
    
    fn get_current_health(&self) -> i32 {
        self.base.get_current_health()
    }
    
    fn get_block(&self) -> i32 {
        self.base.get_block()
    }
    
    fn is_alive(&self) -> bool {
        self.base.is_alive()
    }
    
    fn get_status(&self, status_type: &crate::core::base_state::StatusType) -> i32 {
        self.base.get_status(status_type)
    }
    
    fn get_all_statuses(&self) -> &Vec<crate::core::base_state::Status> {
        self.base.get_all_statuses()
    }
    
    fn add_status(&mut self, status_type: crate::core::base_state::StatusType, stacks: i32) {
        self.base.add_status(status_type, stacks)
    }
    
    fn set_block(&mut self, amount: i32) {
        self.base.set_block(amount)
    }
    
    fn set_health(&mut self, amount: i32) {
        self.base.set_health(amount)
    }
}
