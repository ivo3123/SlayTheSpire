
#[derive(Clone, Debug, PartialEq)]
pub enum StatusType {
    Strength,
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
    fn take_damage(&mut self, damage: i32);
    fn heal(&mut self, amount: i32);
    fn gain_block(&mut self, amount: i32);
    fn add_status(&mut self, status_type: StatusType, stacks: i32);
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
    
    fn take_damage(&mut self, damage: i32) {
        let actual_damage = (damage - self.block).max(0);
        self.block = (self.block - damage).max(0);
        self.current_health -= actual_damage;
    }
    
    fn heal(&mut self, amount: i32) {
        self.current_health = (self.current_health + amount).min(self.max_health);
    }
    
    fn gain_block(&mut self, amount: i32) {
        self.block += amount;
    }
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        if let Some(status) = self.statuses.iter_mut().find(|s| s.status_type == status_type) {
            status.stacks += stacks;
            if status.stacks <= 0 {
                self.statuses.retain(|s| s.status_type != status_type);
            }
        } else if stacks > 0 {
            self.statuses.push(Status::new(status_type, stacks));
        }
    }
}