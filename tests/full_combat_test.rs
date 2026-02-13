use SlayTheSpire::core::{GameState, Player, STSClass, EntityId, State, StatusType, Enemy};
use SlayTheSpire::enemies::Dragonling;
use SlayTheSpire::cards::{strike, defend, inflame, upgrade_card};

#[test]
fn test_complete_combat_scenario() {
    let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 80);
    let mut starting_deck = Vec::new();
    for i in 0..5 {
        starting_deck.push(strike(i, false));
    }
    for i in 5..10 {
        starting_deck.push(defend(i, false));
    }
    
    let enemies = vec![
        Box::new(Dragonling::new()) as Box<dyn Enemy>,
        Box::new(Dragonling::new()) as Box<dyn Enemy>,
    ];
    
    let mut game = GameState::new_with_deck(player, enemies, starting_deck);
    
    assert_eq!(game.player().get_current_health(), 80);
    assert_eq!(game.enemies().len(), 2);
    assert!(!game.is_combat_over());
    
    game.start_player_turn();
    assert_eq!(game.hand().len(), 5);
    assert_eq!(game.player().get_energy(), 3);
    
    if let Some(strike_idx) = game.hand().iter().position(|c| c.id() == "strike") {
        game.play_card(strike_idx, Some(EntityId::Enemy(0))).unwrap();
        assert!(game.enemies()[0].get_current_health() < 50);
    }
    
    game.end_player_turn();
    game.execute_all_enemy_turns();
    
    assert!(game.player().get_current_health() <= 80);
    
    game.start_player_turn();
    
    assert!(!game.is_combat_over());
    assert_eq!(game.living_enemy_count(), 2);
}

#[test]
fn test_victory_condition() {
    let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    assert!(!game.is_combat_over());
    
    game.deal_damage(EntityId::Player, EntityId::Enemy(0), 50);
    
    assert!(game.is_combat_over());
    assert!(game.are_all_enemies_dead());
    assert!(!game.is_player_dead());
}

#[test]
fn test_defeat_condition() {
    let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 10);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    assert!(!game.is_combat_over());
    
    game.deal_damage(EntityId::Enemy(0), EntityId::Player, 20);
    
    assert!(game.is_combat_over());
    assert!(!game.are_all_enemies_dead());
    assert!(game.is_player_dead());
}

#[test]
fn test_upgrade_system_in_combat() {
    let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    
    let strike_regular = strike(100, false);
    let strike_upgraded = upgrade_card(strike_regular);
    
    assert!(strike_upgraded.is_upgraded());
    
    let strike_regular2 = strike(101, false);
    game.add_card_to_hand(strike_regular2);
    
    let strike_upgraded2 = strike(102, true);
    game.add_card_to_hand(strike_upgraded2);
    
    let enemy_hp = game.enemies()[0].get_current_health();
    
    game.play_card(0, Some(EntityId::Enemy(0))).unwrap();
    let damage1 = enemy_hp - game.enemies()[0].get_current_health();
    
    let enemy_hp2 = game.enemies()[0].get_current_health();
    game.play_card(0, Some(EntityId::Enemy(0))).unwrap();
    let damage2 = enemy_hp2 - game.enemies()[0].get_current_health();
    
    assert!(damage2 > damage1);
}

#[test]
fn test_multi_turn_status_effects() {
    let player = Player::new(STSClass::Ironclad, "Hero".to_string(), 100);
    let enemies = vec![Box::new(Dragonling::new()) as Box<dyn Enemy>];
    let mut game = GameState::new(player, enemies);
    
    game.start_player_turn();
    game.add_status(EntityId::Player, StatusType::Strength, 2);
    
    assert_eq!(game.player().get_status(&StatusType::Strength), 2);
    
    game.add_status(EntityId::Enemy(0), StatusType::Vulnerable, 1);
    assert_eq!(game.enemies()[0].get_status(&StatusType::Vulnerable), 1);
    
    game.end_player_turn();
    game.execute_all_enemy_turns();
    
    assert_eq!(game.player().get_status(&StatusType::Strength), 2);
}
