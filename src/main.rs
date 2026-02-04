mod core;
mod enemies;
mod cards;

use core::{Player, STSClass, GameState, EntityId, GameEvent, State};
use cards::Ritual;

fn main() {
    let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 75);
    let mut game = GameState::new(player, vec![]);
    
    game.add_effect(EntityId::Player, Box::new(Ritual { amount: 3 }));
    
    println!("Strength before: {}", game.player.get_status(&core::StatusType::Strength));
    
    game.fire_event(GameEvent::TurnEnded { entity: EntityId::Player });
    
    println!("Strength after: {}", game.player.get_status(&core::StatusType::Strength));
    println!("✅ Effect system working!");
}
