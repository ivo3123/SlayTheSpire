use crate::core::base_state::{BaseState, State, StatusType, Status};

#[derive(Clone, Debug, PartialEq)]
pub enum STSClass {
    Ironclad,
    // Silent,
    // Defect,
    // Watcher,
}

#[derive(Clone, Debug)]
pub struct Player {
    class: STSClass,
    base_state: BaseState,
    max_energy: i32,
    energy: i32,
    hero_ability_used_this_turn: bool,
}

impl Player {
    pub fn new(class: STSClass, name: String, max_health: i32) -> Self {
        let initial_energy = 3;

        Player {
            class: class,
            base_state: BaseState::new(name, max_health),
            max_energy: initial_energy,
            energy: initial_energy,
            hero_ability_used_this_turn: false,
        }
    }
    
    pub fn get_energy(&self) -> i32 {
        self.energy
    }
    
    pub fn get_max_energy(&self) -> i32 {
        self.max_energy
    }
    
    pub fn spend_energy(&mut self, amount: i32) {
        self.energy = (self.energy - amount).max(0);
    }
    
    pub fn refill_energy(&mut self) {
        self.energy = self.max_energy;
    }
    
    pub fn hero_ability_used(&self) -> bool {
        self.hero_ability_used_this_turn
    }
    
    pub fn use_hero_ability(&mut self) {
        self.hero_ability_used_this_turn = true;
    }
    
    pub fn reset_hero_ability(&mut self) {
        self.hero_ability_used_this_turn = false;
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
    
    fn clear_all_statuses(&mut self) {
        self.base_state.clear_all_statuses()
    }
    
    fn clear_all_modifiers(&mut self) {
        self.base_state.clear_all_modifiers()
    }
}
