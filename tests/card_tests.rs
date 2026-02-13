use SlayTheSpire::core::{GameState, Player, STSClass, EntityId, State};
use SlayTheSpire::enemies::Dragonling;
use SlayTheSpire::cards::{strike, defend, inflame, barricade, whirlwind, haste};

#[test]
fn test_card_creation() {
    let strike_card = strike(1, false);
    assert_eq!(strike_card.name(), "Strike");
    assert_eq!(strike_card.get_current_cost().unwrap(), Some(1));
    assert!(!strike_card.is_upgraded());
    assert!(!strike_card.exhaust());
}

#[test]
fn test_upgraded_card() {
    let strike_plus = strike(1, true);
    assert_eq!(strike_plus.name(), "Strike+");
    assert!(strike_plus.is_upgraded());
}

#[test]
fn test_strike_deals_damage() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let strike_card = strike(100, false);
    game.add_card_to_hand(strike_card);
    
    let enemy_hp_before = game.enemies()[0].get_current_health();
    game.play_card(0, Some(EntityId::Enemy(0))).unwrap();
    let enemy_hp_after = game.enemies()[0].get_current_health();
    
    assert_eq!(enemy_hp_before - enemy_hp_after, 6);
}

#[test]
fn test_defend_gains_block() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let defend_card = defend(100, false);
    game.add_card_to_hand(defend_card);
    
    assert_eq!(game.player().get_block(), 0);
    game.play_card(0, None).unwrap();
    assert_eq!(game.player().get_block(), 5);
}

#[test]
fn test_upgraded_defend_more_block() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let defend_plus = defend(100, true);
    game.add_card_to_hand(defend_plus);
    
    game.play_card(0, None).unwrap();
    assert_eq!(game.player().get_block(), 12);
}

#[test]
fn test_exhaust_card() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let haste_card = haste(100, false);
    assert!(haste_card.exhaust());
    game.add_card_to_hand(haste_card);
    
    game.play_card(0, None).unwrap();
    
    assert_eq!(game.exhaust_pile().len(), 1);
    assert_eq!(game.discard_pile().len(), 0);
}

#[test]
fn test_x_cost_whirlwind() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![
        Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>,
        Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>,
    ];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let whirlwind_card = whirlwind(100, false);
    game.add_card_to_hand(whirlwind_card);
    
    let energy_before = game.player().get_energy();
    game.play_card(0, None).unwrap();
    
    assert_eq!(game.player().get_energy(), 0);
    assert_eq!(game.enemies()[0].get_current_health(), 50 - (5 * energy_before));
    assert_eq!(game.enemies()[1].get_current_health(), 50 - (5 * energy_before));
}

#[test]
fn test_card_targeting() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![
        Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>,
        Box::new(Dragonling::new()) as Box<dyn SlayTheSpire::core::Enemy>,
    ];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let strike_card = strike(100, false);
    game.add_card_to_hand(strike_card);
    
    game.play_card(0, Some(EntityId::Enemy(0))).unwrap();
    
    assert_eq!(game.enemies()[0].get_current_health(), 44);
    assert_eq!(game.enemies()[1].get_current_health(), 50);
}
