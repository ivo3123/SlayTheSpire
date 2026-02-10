use crate::core::base_state::{BaseState, State, StatusType, Status};
use crate::core::action::Intent;

pub trait Enemy: State {
    fn get_intent(&self, turn_count: usize) -> Intent;
}

#[derive(Clone, Debug)]
pub struct BaseEnemy {
    base_state: BaseState,
    id: String,
}

impl BaseEnemy {
    pub fn new(id: String, name: String, max_health: i32) -> Self {
        BaseEnemy {
            base_state: BaseState::new(name, max_health),
            id,
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl State for BaseEnemy {
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
    
    fn get_all_statuses_mut(&mut self) -> &mut Vec<Status> {
        self.base_state.get_all_statuses_mut()
    }
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        self.base_state.add_status(status_type, stacks)
    }
    
    fn reduce_status(&mut self, status_type: StatusType, amount: i32) {
        self.base_state.reduce_status(status_type, amount)
    }
    
    fn set_block(&mut self, amount: i32) {
        self.base_state.set_block(amount)
    }
    
    fn set_health(&mut self, amount: i32) {
        self.base_state.set_health(amount)
    }
    
    fn has_modifier(&self, modifier: &crate::core::base_state::Modifier) -> bool {
        self.base_state.has_modifier(modifier)
    }
    
    fn add_modifier(&mut self, modifier: crate::core::base_state::Modifier) {
        self.base_state.add_modifier(modifier)
    }
    
    fn remove_modifier(&mut self, modifier: &crate::core::base_state::Modifier) {
        self.base_state.remove_modifier(modifier)
    }
    
    fn remove_expired_statuses(&mut self) {
        self.base_state.remove_expired_statuses()
    }
    
    fn decay_debuffs(&mut self) {
        self.base_state.decay_debuffs()
    }
}
