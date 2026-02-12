use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::core::base_state::Modifier;
use crate::cards::card_effects::AddModifierAction;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct BarricadeConfig {
    name: String,
    cost: i32,
    description: String,
}

pub fn barricade(instance_id: u32) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/barricade.json")
        .expect("Failed to read barricade.json");
    let config: BarricadeConfig = serde_json::from_str(&config_str).unwrap();
    
    Card::new(
        instance_id,
        "barricade".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Power,
        CardTargeting::None,
        vec![Box::new(AddModifierAction {
            modifier: Modifier::RetainBlock,
        })],
        config.description,
    )
}
