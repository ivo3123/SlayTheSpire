use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::{DrawCardsAction, ApplyStatusAction};
use crate::core::base_state::StatusType;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct HasteConfig {
    name: String,
    cost: i32,
    draw: usize,
    vulnerable: i32,
    description: String,
}

#[derive(Deserialize)]
struct HasteFullConfig {
    regular: HasteConfig,
    upgraded: HasteConfig,
}

pub fn haste(instance_id: u32, upgraded: bool) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/haste.json")
        .expect("Failed to read haste.json");
    let full_config: HasteFullConfig = serde_json::from_str(&config_str).unwrap();
    let config = if upgraded { full_config.upgraded } else { full_config.regular };
    
    Card::new(
        instance_id,
        "haste".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Skill,
        CardTargeting::SingleEnemy,
        vec![
            Box::new(DrawCardsAction { count: config.draw }),
            Box::new(ApplyStatusAction { 
                status_type: StatusType::Vulnerable, 
                stacks: config.vulnerable 
            }),
        ],
        config.description
            .replace("{draw}", &config.draw.to_string())
            .replace("{vulnerable}", &config.vulnerable.to_string()),
        upgraded,
        upgrade,
        true,
    )
}

pub fn upgrade(instance_id: u32) -> Card {
    haste(instance_id, true)
}
