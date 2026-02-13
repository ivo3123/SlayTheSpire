use crate::core::enemy::{BaseEnemy, Enemy};
use crate::core::base_state::State;
use crate::core::action::Intent;
use crate::cards::{DamageEffect, BlockEffect};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct DragonlingConfig {
    name: String,
    max_health: i32,
}

pub struct Dragonling {
    base: BaseEnemy,
}

impl Dragonling {
    pub fn new() -> Self {
        let config_str = fs::read_to_string("assets/enemies/data/dragonling.json")
            .expect("Failed to read dragonling.json");
        let config: DragonlingConfig = serde_json::from_str(&config_str).unwrap();
        
        Dragonling {
            base: BaseEnemy::new("dragonling".to_string(), config.name, config.max_health),
        }
    }
}

impl Enemy for Dragonling {
    fn get_intent(&self, turn_count: usize) -> Intent {
        match turn_count % 4 {
            0 => {
                let amount = 8;
                Intent::new(
                    vec![Box::new(DamageEffect { amount })],
                    format!("Attack for {}", amount),
                )
            },
            1 => {
                let amount = 6;
                Intent::new(
                    vec![Box::new(BlockEffect { amount })],
                    format!("Gain {} Block", amount),
                )
            },
            2 | 3 => {
                let amount = 6;
                Intent::new(
                    vec![Box::new(DamageEffect { amount })],
                    format!("Attack for {}", amount),
                )
            },
            _ => unreachable!(),
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
    
    fn get_all_statuses_mut(&mut self) -> &mut Vec<crate::core::base_state::Status> {
        self.base.get_all_statuses_mut()
    }
    
    fn add_status(&mut self, status_type: crate::core::base_state::StatusType, stacks: i32) {
        self.base.add_status(status_type, stacks)
    }
    
    fn reduce_status(&mut self, status_type: crate::core::base_state::StatusType, amount: i32) {
        self.base.reduce_status(status_type, amount)
    }
    
    fn set_block(&mut self, amount: i32) {
        self.base.set_block(amount)
    }
    
    fn set_health(&mut self, amount: i32) {
        self.base.set_health(amount)
    }
    
    fn has_modifier(&self, modifier: &crate::core::base_state::Modifier) -> bool {
        self.base.has_modifier(modifier)
    }
    
    fn add_modifier(&mut self, modifier: crate::core::base_state::Modifier) {
        self.base.add_modifier(modifier)
    }
    
    fn remove_modifier(&mut self, modifier: &crate::core::base_state::Modifier) {
        self.base.remove_modifier(modifier)
    }
    
    fn remove_expired_statuses(&mut self) {
        self.base.remove_expired_statuses()
    }
    
    fn decay_debuffs(&mut self) {
        self.base.decay_debuffs()
    }
    
    fn clear_all_statuses(&mut self) {
        self.base.clear_all_statuses()
    }
    
    fn clear_all_modifiers(&mut self) {
        self.base.clear_all_modifiers()
    }
}
