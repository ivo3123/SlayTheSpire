use SlayTheSpire::core::{GameState, Player, STSClass, State, Enemy};
use SlayTheSpire::enemies::Dragonling;
use SlayTheSpire::cards::{strike, defend, inflame};

#[test]
fn test_hero_ability_basic() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), defend(2, false), inflame(3, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    
    assert_eq!(game.hand().len(), 3);
    assert_eq!(game.player().get_energy(), 3);
    assert_eq!(game.player().get_current_health(), 100);
    assert!(!game.player().hero_ability_used());
    
    game.use_hero_ability().unwrap();
    
    assert_eq!(game.hand().len(), 2);
    assert_eq!(game.player().get_energy(), 2);
    assert_eq!(game.player().get_current_health(), 98);
    assert!(game.player().hero_ability_used());
    assert_eq!(game.exhaust_pile().len(), 1);
}

#[test]
fn test_hero_ability_exhausts_rightmost() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), defend(2, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    
    let rightmost_name = game.hand().last().unwrap().name().to_string();
    
    game.use_hero_ability().unwrap();
    
    let exhausted_name = game.exhaust_pile()[0].name().to_string();
    assert_eq!(exhausted_name, rightmost_name);
}

#[test]
fn test_hero_ability_cannot_use_twice() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), defend(2, false), inflame(3, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    
    game.use_hero_ability().unwrap();
    let result = game.use_hero_ability();
    
    assert!(result.is_err());
}

#[test]
fn test_hero_ability_requires_energy() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    game.player_mut().spend_energy(3);
    
    let result = game.use_hero_ability();
    assert!(result.is_err());
}

#[test]
fn test_hero_ability_requires_cards_in_hand() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    game.discard_hand();
    
    let result = game.use_hero_ability();
    assert!(result.is_err());
}

#[test]
fn test_hero_ability_adds_upgraded_next_turn() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), defend(2, false), inflame(3, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    
    let rightmost_name = game.hand().last().unwrap().name().to_string();
    
    game.use_hero_ability().unwrap();
    
    game.end_player_turn();
    game.execute_all_enemy_turns();
    game.start_player_turn();
    
    let has_upgraded = game.hand().iter().any(|c| c.is_upgraded() && c.name().contains(&rightmost_name));
    assert!(has_upgraded);
}

#[test]
fn test_hero_ability_resets_each_turn() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), defend(2, false), inflame(3, false), strike(4, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    game.use_hero_ability().unwrap();
    assert!(game.player().hero_ability_used());
    
    game.end_player_turn();
    game.execute_all_enemy_turns();
    game.start_player_turn();
    
    assert!(!game.player().hero_ability_used());
    
    let result = game.use_hero_ability();
    assert!(result.is_ok());
}
