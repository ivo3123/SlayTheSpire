use crate::core::{Player, card::Card};
use crate::core::base_state::{StatusType, Modifier, State};
use crate::core::enemy::Enemy;
use crate::core::action::Intent;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityId {
    Player,
    Enemy(usize),
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    TurnStarted { entity: EntityId },
    TurnEnded { entity: EntityId },
    CardPlayed { card: u32, source: EntityId },
    EnemyAction { enemy: EntityId },
    DamageDealt { source: EntityId, target: EntityId, amount: i32 },
    BlockGained { entity: EntityId, amount: i32 },
}

pub struct GameState {
    pub player: Player,
    pub enemies: Vec<Box<dyn Enemy>>,
    pub effects: Vec<(EntityId, Box<dyn crate::core::effects::Effect>)>,
    
    // Deck management
    pub draw_pile: Vec<Card>,
    pub hand: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub exhaust_pile: Vec<Card>,
    
    card_registry: HashMap<u32, Card>,
    turn_count: usize,
}

impl GameState {
    pub fn new(player: Player, enemies: Vec<Box<dyn Enemy>>) -> Self {
        GameState {
            player,
            enemies,
            effects: Vec::new(),
            draw_pile: Vec::new(),
            hand: Vec::new(),
            discard_pile: Vec::new(),
            exhaust_pile: Vec::new(),
            card_registry: HashMap::new(),
            turn_count: 0,
        }
    }
    
    pub fn new_with_deck(player: Player, enemies: Vec<Box<dyn Enemy>>, starting_deck: Vec<Card>) -> Self {
        let mut game = Self::new(player, enemies);
        game.draw_pile = starting_deck;
        game.shuffle_draw_pile();
        game
    }
    
    pub fn add_effect(&mut self, owner: EntityId, effect: Box<dyn crate::core::effects::Effect>) {
        self.effects.push((owner, effect));
    }
    
    pub fn fire_event(&mut self, event: GameEvent) {
        let mut effects = std::mem::take(&mut self.effects);
        
        for (owner, effect) in effects.iter_mut() {
            let mut ctx = crate::core::effects::GameContext {
                owner: *owner,
                game_state: self,
            };
            effect.on_event(&event, &mut ctx);
        }
        
        effects.retain(|(_, e)| !e.should_remove());
        self.effects = effects;
    }
    
    fn get_status(&self, entity: EntityId, status_type: StatusType) -> i32 {
        match entity {
            EntityId::Player => self.player.get_status(&status_type),
            EntityId::Enemy(id) => {
                self.enemies.get(id)
                    .map(|e| e.get_status(&status_type))
                    .unwrap_or(0)
            }
        }
    }
    
    fn get_block(&self, entity: EntityId) -> i32 {
        match entity {
            EntityId::Player => self.player.get_block(),
            EntityId::Enemy(id) => {
                self.enemies.get(id)
                    .map(|e| e.get_block())
                    .unwrap_or(0)
            }
        }
    }
    
    fn modify_block(&mut self, entity: EntityId, delta: i32) {
        match entity {
            EntityId::Player => {
                let new_block = (self.player.get_block() + delta).max(0);
                self.player.set_block(new_block);
            }
            EntityId::Enemy(id) => {
                if let Some(enemy) = self.enemies.get_mut(id) {
                    let new_block = (enemy.get_block() + delta).max(0);
                    enemy.set_block(new_block);
                }
            }
        }
    }
    
    fn modify_hp(&mut self, entity: EntityId, delta: i32) {
        match entity {
            EntityId::Player => {
                let new_hp = (self.player.get_current_health() + delta)
                    .clamp(0, self.player.get_max_health());
                self.player.set_health(new_hp);
            }
            EntityId::Enemy(id) => {
                if let Some(enemy) = self.enemies.get_mut(id) {
                    let new_hp = (enemy.get_current_health() + delta)
                        .clamp(0, enemy.get_max_health());
                    enemy.set_health(new_hp);
                }
            }
        }
    }

    pub fn deal_damage(
        &mut self,
        source: EntityId,
        target: EntityId,
        base_damage: i32,
    ) {
        let mut dmg = base_damage;
        
        // 1. Attacker modifiers
        let strength = self.get_status(source, StatusType::Strength);
        dmg += strength;
        
        let weak = self.get_status(source, StatusType::Weak);
        if weak > 0 {
            dmg = (dmg as f32 * 0.75).floor() as i32;
        }
        
        // 2. Target modifiers
        let vulnerable = self.get_status(target, StatusType::Vulnerable);
        if vulnerable > 0 {
            dmg = (dmg as f32 * 1.5).floor() as i32;
        }
        
        dmg = dmg.max(0);
        
        // 3. Apply block
        let block = self.get_block(target);
        let absorbed = dmg.min(block);
        let final_damage = dmg - absorbed;
        
        self.modify_block(target, -absorbed);
        self.modify_hp(target, -final_damage);
        
        // 4. Fire event
        self.fire_event(GameEvent::DamageDealt {
            source,
            target,
            amount: final_damage,
            // unblocked_damage_taken: ...
        });
    }
    
    pub fn gain_block(&mut self, entity: EntityId, base_block: i32) {
        let mut block = base_block;
        
        // 1. Apply Dexterity
        let dexterity = self.get_status(entity, StatusType::Dexterity);
        block += dexterity;
        
        // 2. Apply Frail (reduces block gain)
        let frail = self.get_status(entity, StatusType::Frail);
        if frail > 0 {
            block = (block as f32 * 0.75).floor() as i32;
        }
        
        block = block.max(0);
        
        self.modify_block(entity, block);
        
        // 3. Fire event
        self.fire_event(GameEvent::BlockGained {
            entity,
            amount: block,
        });
    }
    
    pub fn play_card(&mut self, card: &Card, source: EntityId, target: Option<EntityId>) -> Result<(), String> {
        // Check if player has enough energy
        if let EntityId::Player = source {
            if self.player.get_energy() < card.cost() {
                return Err(format!("Not enough energy: need {}, have {}", card.cost(), self.player.get_energy()));
            }
            self.player.spend_energy(card.cost());
        }
        
        for effect in card.effects() {
            effect.resolve(self, source, target);
        }

        self.fire_event(GameEvent::CardPlayed {
            card: card.instance_id(),
            source,
        });
        
        Ok(())
    }
    
    pub fn shuffle_draw_pile(&mut self) {
        let mut rng = thread_rng();
        self.draw_pile.shuffle(&mut rng);
    }
    
    pub fn draw_card(&mut self) -> Option<Card> {
        // shuffle discard back into draw pile
        if self.draw_pile.is_empty() && !self.discard_pile.is_empty() {
            self.draw_pile.append(&mut self.discard_pile);
            self.shuffle_draw_pile();
        }
        
        self.draw_pile.pop()
    }
    
    pub fn draw_cards(&mut self, count: usize) {
        for _ in 0..count {
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            }
        }
    }
    
    pub fn discard_from_hand(&mut self, index: usize) -> Option<Card> {
        if index < self.hand.len() {
            let card = self.hand.remove(index);
            self.discard_pile.push(card.clone());
            Some(card)
        } else {
            None
        }
    }
    
    pub fn discard_hand(&mut self) {
        self.discard_pile.append(&mut self.hand);
    }
    
    pub fn exhaust_from_hand(&mut self, index: usize) -> Option<Card> {
        if index < self.hand.len() {
            let card = self.hand.remove(index);
            self.exhaust_pile.push(card.clone());
            Some(card)
        } else {
            None
        }
    }
    
    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }
    
    pub fn add_card_to_discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }
    
    pub fn add_card_to_draw_pile(&mut self, card: Card) {
        self.draw_pile.push(card);
    }
    
    pub fn execute_enemy_intent(&mut self, enemy_id: usize, intent: &Intent, target: Option<EntityId>) {
        let source = EntityId::Enemy(enemy_id);
        
        intent.execute(self, source, target);

        self.fire_event(GameEvent::EnemyAction {
            enemy: source,
        });
    }
    
    pub fn start_player_turn(&mut self) {
        // Remove block from previous turn (unless Barricade is active)
        if !self.player.has_modifier(&Modifier::RetainBlock) {
            self.player.set_block(0);
        }
        
        // Refill energy
        self.player.refill_energy();
        
        // Draw cards
        self.draw_cards(5);
        
        // Fire event
        self.fire_event(GameEvent::TurnStarted { entity: EntityId::Player });
    }
    
    pub fn end_player_turn(&mut self) {
        // Fire turn ended event (for effects like Ritual)
        self.fire_event(GameEvent::TurnEnded { entity: EntityId::Player });
        
        // Discard hand unless player has RetainHand modifier (e.g., from a relic)
        if !self.player.has_modifier(&Modifier::RetainHand) {
            self.discard_hand();
        }
        
        // Increment turn counter
        self.turn_count += 1;
    }
    
    pub fn get_turn_count(&self) -> usize {
        self.turn_count
    }
}
