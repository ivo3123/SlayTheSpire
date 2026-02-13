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

#[derive(Deserialize)]
struct DefendFullConfig {
    regular: DefendConfig,
    upgraded: DefendConfig,
}

pub fn defend(instance_id: u32, upgraded: bool) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/defend.json")
        .expect("Failed to read defend.json");
    let full_config: DefendFullConfig = serde_json::from_str(&config_str).unwrap();
    let config = if upgraded { full_config.upgraded } else { full_config.regular };
    
    Card::new(
        instance_id,
        "defend".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Skill,
        CardTargeting::Self_,
        vec![Box::new(BlockEffect { amount: config.block })],
        config.description.replace("{}", &config.block.to_string()),
        upgraded,
        upgrade,
        false,
    )
}

pub fn upgrade(instance_id: u32) -> Card {
    defend(instance_id, true)
}
