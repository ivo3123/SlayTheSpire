
#[derive(Clone, Debug, PartialEq)]
pub enum StatusType {
    Strength,
    Dexterity,
    Vulnerable,
    Weak,
    Poison,
    Frail,
}

/// Enum for game breaking mechanics
#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    RetainHand,
    RetainBlock,
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
    modifiers: Vec<Modifier>,
}

pub trait State {
    fn get_name(&self) -> &str;
    fn get_max_health(&self) -> i32;
    fn get_current_health(&self) -> i32;
    fn get_block(&self) -> i32;
    fn is_alive(&self) -> bool;
    fn get_status(&self, status_type: &StatusType) -> i32;
    fn get_all_statuses(&self) -> &Vec<Status>;
    fn get_all_statuses_mut(&mut self) -> &mut Vec<Status>;
    fn add_status(&mut self, status_type: StatusType, stacks: i32);
    fn reduce_status(&mut self, status_type: StatusType, amount: i32);
    fn set_block(&mut self, amount: i32);
    fn set_health(&mut self, amount: i32);
    fn has_modifier(&self, modifier: &Modifier) -> bool;
    fn add_modifier(&mut self, modifier: Modifier);
    fn remove_modifier(&mut self, modifier: &Modifier);
    fn remove_expired_statuses(&mut self);
    fn decay_debuffs(&mut self);
}

impl BaseState {
    pub fn new(name: String, max_health: i32) -> Self {
        BaseState {
            name,
            max_health,
            current_health: max_health,
            block: 0,
            statuses: Vec::new(),
            modifiers: Vec::new(),
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
    
    fn get_all_statuses_mut(&mut self) -> &mut Vec<Status> {
        &mut self.statuses
    }
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        if let Some(status) = self.statuses.iter_mut().find(|s| s.status_type == status_type) {
            status.stacks += stacks;
        } else {
            self.statuses.push(Status::new(status_type, stacks));
        }
    }
    
    fn reduce_status(&mut self, status_type: StatusType, amount: i32) {
        if let Some(status) = self.statuses.iter_mut().find(|s| s.status_type == status_type) {
            status.stacks -= amount;
        }
        self.remove_expired_statuses();
    }
    
    fn set_block(&mut self, amount: i32) {
        self.block = amount.max(0);
    }
    
    fn set_health(&mut self, amount: i32) {
        self.current_health = amount.clamp(0, self.max_health);
    }
    
    fn has_modifier(&self, modifier: &Modifier) -> bool {
        self.modifiers.contains(modifier)
    }
    
    fn add_modifier(&mut self, modifier: Modifier) {
        if !self.modifiers.contains(&modifier) {
            self.modifiers.push(modifier);
        }
    }
    
    fn remove_modifier(&mut self, modifier: &Modifier) {
        self.modifiers.retain(|m| m != modifier);
    }
    
    fn remove_expired_statuses(&mut self) {
        // Remove statuses with 0 or negative stacks
        self.statuses.retain(|s| s.stacks > 0);
    }
    
    fn decay_debuffs(&mut self) {
        for status in &mut self.statuses {
            match status.status_type {
                StatusType::Vulnerable | StatusType::Weak | StatusType::Frail => {
                    status.stacks -= 1;
                }
                _ => {}
            }
        }
        self.remove_expired_statuses();
    }
}