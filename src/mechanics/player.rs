use crate::mechanics::base_state::{BaseState, State, StatusType, Status};

#[derive(Clone, Debug)]
pub struct Player {
    base_state: BaseState,
    max_energy: i32,
    energy: i32,
}

impl Player {
    pub fn new(name: String, max_health: i32) -> Self {
        Player {
            base_state: BaseState::new(name, max_health),
            max_energy: 3,
            energy: 3,
        }
    }
}

impl State for Player {
    fn get_name(&self) -> &str {
        self.base_state.get_name()
    }
    
    fn get_max_health(&self) -> i32 {
        self.base_state.get_max_health()
    }
    
    fn get_current_health(&self) -> i32 {
        self.base_state.get_current_health()
    }
    
    fn get_block(&self) -> i32 {
        self.base_state.get_block()
    }
    
    fn is_alive(&self) -> bool {
        self.base_state.is_alive()
    }
    
    fn get_status(&self, status_type: &StatusType) -> i32 {
        self.base_state.get_status(status_type)
    }
    
    fn get_all_statuses(&self) -> &Vec<Status> {
        self.base_state.get_all_statuses()
    }
    
    fn take_damage(&mut self, damage: i32) {
        self.base_state.take_damage(damage)
    }
    
    fn heal(&mut self, amount: i32) {
        self.base_state.heal(amount)
    }
    
    fn gain_block(&mut self, amount: i32) {
        self.base_state.gain_block(amount)
    }
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        self.base_state.add_status(status_type, stacks)
    }
}
