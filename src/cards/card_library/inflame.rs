use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::ApplyStatusAction;
use crate::core::base_state::StatusType;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct InflameConfig {
    name: String,
    cost: i32,
    strength: i32,
    description: String,
}

#[derive(Deserialize)]
struct InflameFullConfig {
    regular: InflameConfig,
    upgraded: InflameConfig,
}

pub fn inflame(instance_id: u32, upgraded: bool) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/inflame.json")
        .expect("Failed to read inflame.json");
    let full_config: InflameFullConfig = serde_json::from_str(&config_str).unwrap();
    let config = if upgraded { full_config.upgraded } else { full_config.regular };
    
    Card::new(
        instance_id,
        "inflame".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Power,
        CardTargeting::Self_,
        vec![Box::new(ApplyStatusAction {
            status_type: StatusType::Strength,
            stacks: config.strength,
        })],
        config.description.replace("{}", &config.strength.to_string()),
        upgraded,
        upgrade,
        true,  // Power cards exhaust after use
    )
}

pub fn upgrade(instance_id: u32) -> Card {
    inflame(instance_id, true)
}
