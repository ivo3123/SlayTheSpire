use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::{ApplyEffect, Ritual};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct InflameConfig {
    name: String,
    cost: i32,
    strength: i32,
    description: String,
}

pub fn inflame(instance_id: u32) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/inflame.json")
        .expect("Failed to read inflame.json");
    let config: InflameConfig = serde_json::from_str(&config_str).unwrap();
    
    Card::new(
        instance_id,
        "inflame".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Power,
        CardTargeting::Self_,
        vec![Box::new(ApplyEffect {
            effect: Box::new(Ritual { amount: config.strength }),
        })],
        config.description.replace("{}", &config.strength.to_string()),
    )
}
