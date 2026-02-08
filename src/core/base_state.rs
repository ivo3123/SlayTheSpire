
#[derive(Clone, Debug, PartialEq)]
pub enum StatusType {
    Strength,
    Dexterity,
    Vulnerable,
    Weak,
    Poison,
    Frail,
}

#[derive(Clone, Debug)]
pub struct Status {
    pub status_type: StatusType,
    pub stacks: i32,
}

impl Status {
    pub fn new(status_type: StatusType, stacks: i32) -> Self {
        Status { status_type, stacks }
    }
}

#[derive(Clone, Debug)]
pub struct BaseState {
    name: String,
    max_health: i32,
    current_health: i32,
    block: i32,
    statuses: Vec<Status>,
}

pub trait State {
    fn get_name(&self) -> &str;
    fn get_max_health(&self) -> i32;
    fn get_current_health(&self) -> i32;
    fn get_block(&self) -> i32;
    fn is_alive(&self) -> bool;
    fn get_status(&self, status_type: &StatusType) -> i32;
    fn get_all_statuses(&self) -> &Vec<Status>;
    fn add_status(&mut self, status_type: StatusType, stacks: i32);
    fn set_block(&mut self, amount: i32);
    fn set_health(&mut self, amount: i32);
}

impl BaseState {
    pub fn new(name: String, max_health: i32) -> Self {
        BaseState {
            name,
            max_health,
            current_health: max_health,
            block: 0,
            statuses: Vec::new(),
        }
    }
}

impl State for BaseState {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_max_health(&self) -> i32 {
        self.max_health
    }
    
    fn get_current_health(&self) -> i32 {
        self.current_health
    }
    
    fn get_block(&self) -> i32 {
        self.block
    }
    
    fn is_alive(&self) -> bool {
        self.current_health > 0
    }
    
    fn get_status(&self, status_type: &StatusType) -> i32 {
        self.statuses
            .iter()
            .find(|s| &s.status_type == status_type)
            .map(|s| s.stacks)
            .unwrap_or(0)
    }
    
    fn get_all_statuses(&self) -> &Vec<Status> {
        &self.statuses
    }
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        if let Some(status) = self.statuses.iter_mut().find(|s| s.status_type == status_type) {
            status.stacks += stacks;
        } else {
            self.statuses.push(Status::new(status_type, stacks));
        }
    }
    
    fn set_block(&mut self, amount: i32) {
        self.block = amount.max(0);
    }
    
    fn set_health(&mut self, amount: i32) {
        self.current_health = amount.clamp(0, self.max_health);
    }
}