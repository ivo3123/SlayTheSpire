use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::DamageEffect;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct StrikeConfig {
    name: String,
    cost: i32,
    damage: i32,
    description: String,
}

pub fn strike(instance_id: u32) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/strike.json")
        .expect("Failed to read strike.json");
    let config: StrikeConfig = serde_json::from_str(&config_str).unwrap();
    
    Card::new(
        instance_id,
        "strike".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Attack,
        CardTargeting::SingleEnemy,
        vec![Box::new(DamageEffect { amount: config.damage })],
        config.description.replace("{}", &config.damage.to_string()),
    )
}
