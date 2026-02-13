use SlayTheSpire::core::{GameState, Player, STSClass, EntityId, State, StatusType, Enemy};
use SlayTheSpire::enemies::Dragonling;
use SlayTheSpire::cards::{strike, defend};

#[test]
fn test_energy_system() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    assert_eq!(game.player().get_energy(), 3);
    
    let strike_card = strike(100, false);
    game.add_card_to_hand(strike_card);
    game.play_card(0, Some(EntityId::Enemy(0))).unwrap();
    
    assert_eq!(game.player().get_energy(), 2);
}

#[test]
fn test_insufficient_energy() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    game.player_mut().spend_energy(3);
    
    let strike_card = strike(100, false);
    game.add_card_to_hand(strike_card);
    
    let result = game.play_card(0, Some(EntityId::Enemy(0)));
    assert!(result.is_err());
}

#[test]
fn test_block_absorbs_damage() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let defend_card = defend(100, false);
    game.add_card_to_hand(defend_card);
    game.play_card(0, None).unwrap();
    
    assert_eq!(game.player().get_block(), 5);
    
    game.deal_damage(EntityId::Enemy(0), EntityId::Player, 3);
    
    assert_eq!(game.player().get_block(), 2);
    assert_eq!(game.player().get_current_health(), 100);
}

#[test]
fn test_damage_exceeds_block() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    let defend_card = defend(100, false);
    game.add_card_to_hand(defend_card);
    game.play_card(0, None).unwrap();
    
    assert_eq!(game.player().get_block(), 5);
    
    game.deal_damage(EntityId::Enemy(0), EntityId::Player, 10);
    
    assert_eq!(game.player().get_block(), 0);
    assert_eq!(game.player().get_current_health(), 95);
}

#[test]
fn test_draw_and_discard() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), strike(2, false), defend(3, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    assert_eq!(game.draw_pile().len(), 3);
    assert_eq!(game.hand().len(), 0);
    
    game.start_player_turn();
    
    assert_eq!(game.hand().len(), 3);
    assert_eq!(game.draw_pile().len(), 0);
    
    game.end_player_turn();
    
    assert_eq!(game.hand().len(), 0);
    assert_eq!(game.discard_pile().len(), 3);
}

#[test]
fn test_shuffle_when_draw_pile_empty() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let deck = vec![strike(1, false), strike(2, false)];
    let mut game = GameState::new_with_deck(player, enemies, deck);
    
    game.start_player_turn();
    assert_eq!(game.hand().len(), 2);
    
    game.end_player_turn();
    game.execute_all_enemy_turns();
    
    game.start_player_turn();
    assert_eq!(game.hand().len(), 2);
}

#[test]
fn test_vulnerable_increases_damage() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.deal_damage(EntityId::Player, EntityId::Enemy(0), 10);
    assert_eq!(game.enemies()[0].get_current_health(), 40);
    
    game.add_status(EntityId::Enemy(0), StatusType::Vulnerable, 1);
    
    game.deal_damage(EntityId::Player, EntityId::Enemy(0), 10);
    assert_eq!(game.enemies()[0].get_current_health(), 25);
}

#[test]
fn test_weak_reduces_damage() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.deal_damage(EntityId::Enemy(0), EntityId::Player, 10);
    let hp_after_normal = game.player().get_current_health();
    assert_eq!(hp_after_normal, 90);
    
    game.add_status(EntityId::Enemy(0), StatusType::Weak, 1);
    
    game.deal_damage(EntityId::Enemy(0), EntityId::Player, 10);
    let hp_after_weak = game.player().get_current_health();
    assert_eq!(hp_after_weak, 83); // 10 * 0.75 = 7.5, floor to 7 damage
}

#[test]
fn test_strength_increases_damage() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.add_status(EntityId::Player, StatusType::Strength, 3);
    
    game.deal_damage(EntityId::Player, EntityId::Enemy(0), 10);
    assert_eq!(game.enemies()[0].get_current_health(), 37);
}

#[test]
fn test_death_removes_enemies() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![
        Box::new(Dragonling::new()) as Box<dyn Enemy>,
        Box::new(Dragonling::new()) as Box<dyn Enemy>,
    ];
    let mut game = GameState::new(player, enemies);
    
    assert_eq!(game.enemies().len(), 2);
    
    game.deal_damage(EntityId::Player, EntityId::Enemy(0), 999);
    game.remove_dead_enemies();
    
    assert_eq!(game.enemies().len(), 1);
    assert_eq!(game.living_enemy_count(), 1);
}

#[test]
fn test_combat_over_when_all_enemies_dead() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    assert!(!game.is_combat_over());
    
    game.deal_damage(EntityId::Player, EntityId::Enemy(0), 999);
    
    assert!(game.are_all_enemies_dead());
    assert!(game.is_combat_over());
}

#[test]
fn test_combat_over_when_player_dead() {
    let player = Player::new(STSClass::Ironclad, "TestHero".to_string(), 10);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    assert!(!game.is_combat_over());
    
    game.deal_damage(EntityId::Enemy(0), EntityId::Player, 999);
    
    assert!(game.is_player_dead());
    assert!(game.is_combat_over());
}
