use crate::core::base_state::{BaseState, State, StatusType, Status};

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
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        self.base_state.add_status(status_type, stacks)
    }
}
