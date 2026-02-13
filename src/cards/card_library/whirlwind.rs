use crate::core::card::{Card, CardType, Cost, CardTargeting};
use crate::cards::card_effects::XDamageEffect;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct WhirlwindConfig {
    name: String,
    damage_per_energy: i32,
    description: String,
}

#[derive(Deserialize)]
struct WhirlwindFullConfig {
    regular: WhirlwindConfig,
    upgraded: WhirlwindConfig,
}

pub fn whirlwind(instance_id: u32, upgraded: bool) -> Card {
    let config_str = fs::read_to_string("assets/cards/data/whirlwind.json")
        .expect("Failed to read whirlwind.json");
    let full_config: WhirlwindFullConfig = serde_json::from_str(&config_str).unwrap();
    let config = if upgraded { full_config.upgraded } else { full_config.regular };
    
    Card::new(
        instance_id,
        "whirlwind".to_string(),
        config.name,
        Cost::X,
        CardType::Attack,
        CardTargeting::AllEnemies,
        vec![Box::new(XDamageEffect { damage_per_energy: config.damage_per_energy })],
        config.description.replace("{}", &config.damage_per_energy.to_string()),
        upgraded,
        upgrade,
        false,
    )
}

pub fn upgrade(instance_id: u32) -> Card {
    whirlwind(instance_id, true)
}
