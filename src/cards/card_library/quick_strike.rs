use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::{DamageEffect, ApplyEffect, EnergyNextTurnEffect};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct QuickStrikeConfig {
    name: String,
    cost: i32,
    damage: i32,
    energy_next_turn: i32,
    description: String,
}

#[derive(Deserialize)]
struct QuickStrikeFullConfig {
    regular: QuickStrikeConfig,
    upgraded: QuickStrikeConfig,
}

pub fn quick_strike(instance_id: u32, upgraded: bool) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/quick_strike.json")
        .expect("Failed to read quick_strike.json");
    let full_config: QuickStrikeFullConfig = serde_json::from_str(&config_str).unwrap();
    let config = if upgraded { full_config.upgraded } else { full_config.regular };
    
    Card::new(
        instance_id,
        "quick_strike".to_string(),
        config.name,
        Cost::Fixed(config.cost),
        CardType::Attack,
        CardTargeting::SingleEnemy,
        vec![
            Box::new(DamageEffect { amount: config.damage }),
            Box::new(ApplyEffect {
                effect: Box::new(EnergyNextTurnEffect::new(config.energy_next_turn)),
            }),
        ],
        config.description
            .replace("{damage}", &config.damage.to_string())
            .replace("{energy}", &config.energy_next_turn.to_string()),
        upgraded,
        upgrade,
        false,
    )
}

pub fn upgrade(instance_id: u32) -> Card {
    quick_strike(instance_id, true)
}
