use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::BlockEffect;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct DefendConfig {
    name: String,
    cost: i32,
    block: i32,
    description: String,
}

pub fn defend(instance_id: u32) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/defend.json")
        .expect("Failed to read defend.json");
    let config: DefendConfig = serde_json::from_str(&config_str).unwrap();
    
    Card::new(
        instance_id,
        "defend".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Skill,
        CardTargeting::Self_,
        vec![Box::new(BlockEffect { amount: config.block })],
        config.description.replace("{}", &config.block.to_string()),
    )
}
