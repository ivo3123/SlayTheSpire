use crate::core::{Player, card::Card};
use crate::core::base_state::{StatusType, Modifier, State};
use crate::core::enemy::Enemy;
use crate::core::action::Intent;
use crate::core::effects::Effect;
use crate::core::card::CardTargeting;
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

#[derive(Clone, Debug)]
pub struct TurnRecord {
    pub turn_number: usize,
    pub cards_played: Vec<u32>,
    pub enemy_intents: Vec<(usize, String)>,
}

impl TurnRecord {
    fn new(turn_number: usize) -> Self {
        TurnRecord {
            turn_number,
            cards_played: Vec::new(),
            enemy_intents: Vec::new(),
        }
    }
}

pub struct GameState {
    player: Player,
    enemies: Vec<Box<dyn Enemy>>,
    effects: Vec<(EntityId, Box<dyn Effect>)>,
    
    draw_pile: Vec<Card>,
    hand: Vec<Card>,
    discard_pile: Vec<Card>,
    exhaust_pile: Vec<Card>,
    
    turn_history: Vec<TurnRecord>,
    current_turn_record: TurnRecord,
    turn_count: usize,
    
    pending_upgraded_card: Option<Card>,
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
            turn_history: Vec::new(),
            current_turn_record: TurnRecord::new(0),
            turn_count: 0,
            pending_upgraded_card: None,
        }
    }
    
    pub fn player(&self) -> &Player {
        &self.player
    }
    
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
    
    pub fn enemies(&self) -> &[Box<dyn Enemy>] {
        &self.enemies
    }
    
    pub fn enemies_mut(&mut self) -> &mut Vec<Box<dyn Enemy>> {
        &mut self.enemies
    }
    
    pub fn get_all_living_enemies(&self) -> Vec<EntityId> {
        self.enemies
            .iter()
            .enumerate()
            .filter(|(_, enemy)| enemy.is_alive())
            .map(|(id, _)| EntityId::Enemy(id))
            .collect()
    }
    
    pub fn get_all_enemy_ids(&self) -> Vec<EntityId> {
        (0..self.enemies.len())
            .map(|id| EntityId::Enemy(id))
            .collect()
    }
    
    pub fn draw_pile(&self) -> &[Card] {
        &self.draw_pile
    }
    
    pub fn hand(&self) -> &[Card] {
        &self.hand
    }
    
    pub fn discard_pile(&self) -> &[Card] {
        &self.discard_pile
    }
    
    pub fn exhaust_pile(&self) -> &[Card] {
        &self.exhaust_pile
    }
    
    pub fn turn_history(&self) -> &[TurnRecord] {
        &self.turn_history
    }
    
    pub fn current_turn_record(&self) -> &TurnRecord {
        &self.current_turn_record
    }
    
    pub fn cards_played_this_turn(&self) -> usize {
        self.current_turn_record.cards_played.len()
    }
    
    pub fn is_first_card_this_turn(&self) -> bool {
        self.current_turn_record.cards_played.is_empty()
    }
    
    pub fn record_enemy_intent(&mut self, enemy_id: usize, intent_description: String) {
        self.current_turn_record.enemy_intents.push((enemy_id, intent_description));
    }
    
    pub fn new_with_deck(player: Player, enemies: Vec<Box<dyn Enemy>>, starting_deck: Vec<Card>) -> Self {
        let mut game = Self::new(player, enemies);
        game.draw_pile = starting_deck;
        game.shuffle_draw_pile();
        game
    }
    
    pub fn add_effect(&mut self, owner: EntityId, effect: Box<dyn Effect>) {
        self.effects.push((owner, effect));
    }
    
    pub fn fire_event(&mut self, event: GameEvent) {
        let mut effects = std::mem::take(&mut self.effects);
        
        for (owner, effect) in effects.iter_mut() {
            effect.on_event(&event, *owner, self);
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
    
    pub fn add_status(&mut self, entity: EntityId, status_type: StatusType, stacks: i32) {
        match entity {
            EntityId::Player => {
                self.player.add_status(status_type, stacks);
            }
            EntityId::Enemy(id) => {
                if let Some(enemy) = self.enemies.get_mut(id) {
                    enemy.add_status(status_type, stacks);
                }
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
        
        let strength = self.get_status(source, StatusType::Strength);
        dmg += strength;
        
        let weak = self.get_status(source, StatusType::Weak);
        if weak > 0 {
            dmg = (dmg as f32 * 0.75).floor() as i32;
        }
        
        let vulnerable = self.get_status(target, StatusType::Vulnerable);
        if vulnerable > 0 {
            dmg = (dmg as f32 * 1.5).floor() as i32;
        }
        
        dmg = dmg.max(0);
        
        let block = self.get_block(target);
        let absorbed = dmg.min(block);
        let final_damage = dmg - absorbed;
        
        self.modify_block(target, -absorbed);
        self.modify_hp(target, -final_damage);
        
        self.fire_event(GameEvent::DamageDealt {
            source,
            target,
            amount: final_damage,
        });
    }
    
    pub fn gain_block(&mut self, entity: EntityId, base_block: i32) {
        let mut block = base_block;
        
        let dexterity = self.get_status(entity, StatusType::Dexterity);
        block += dexterity;
        
        let frail = self.get_status(entity, StatusType::Frail);
        if frail > 0 {
            block = (block as f32 * 0.75).floor() as i32;
        }
        
        block = block.max(0);
        
        self.modify_block(entity, block);
        
        self.fire_event(GameEvent::BlockGained {
            entity,
            amount: block,
        });
    }
    
    pub fn play_card(&mut self, hand_index: usize, target: Option<EntityId>) -> Result<(), String> {
        if hand_index >= self.hand.len() {
            return Err("Card index out of bounds".to_string());
        }
        
        let card = &self.hand[hand_index];
        let targeting = card.targeting();
        
        let actual_targets: Vec<EntityId> = match targeting {
            CardTargeting::SingleEnemy => {
                match target {
                    Some(EntityId::Enemy(id)) => {
                        if id >= self.enemies.len() || self.enemies[id].get_current_health() == 0 {
                            return Err("Invalid enemy target".to_string());
                        }
                        vec![EntityId::Enemy(id)]
                    }
                    _ => return Err("This card requires a single enemy target".to_string()),
                }
            }
            CardTargeting::AllEnemies => {
                self.get_all_living_enemies()
            }
            CardTargeting::Self_ => {
                vec![EntityId::Player]
            }
            CardTargeting::None => {
                vec![]
            }
        };
        
        let energy_spent = if let EntityId::Player = EntityId::Player {
            match card.get_current_cost() {
                Ok(Some(cost)) => {
                    if self.player.get_energy() < cost {
                        return Err(format!("Not enough energy: need {}, have {}", cost, self.player.get_energy()));
                    }
                    self.player.spend_energy(cost);
                    None
                }
                Ok(None) => {
                    let energy = self.player.get_energy();
                    self.player.spend_energy(energy);
                    Some(energy)
                }
                Err(e) => return Err(e),
            }
        } else {
            None
        };
        
        let card = self.remove_from_hand(hand_index).unwrap();
        
        self.current_turn_record.cards_played.push(card.instance_id());
        
        for effect in card.effects() {
            effect.resolve(self, EntityId::Player, &actual_targets, energy_spent);
        }

        self.fire_event(GameEvent::CardPlayed {
            card: card.instance_id(),
            source: EntityId::Player,
        });
        
        if card.exhaust() {
            self.exhaust_pile.push(card);
        } else {
            self.add_card_to_discard(card);
        }
        
        self.remove_dead_enemies();
        
        Ok(())
    }
    
    pub fn shuffle_draw_pile(&mut self) {
        let mut rng = thread_rng();
        self.draw_pile.shuffle(&mut rng);
    }
    
    pub fn draw_card(&mut self) -> Option<Card> {
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
    
    pub fn remove_from_hand(&mut self, index: usize) -> Option<Card> {
        if index < self.hand.len() {
            Some(self.hand.remove(index))
        } else {
            None
        }
    }
    
    pub fn find_in_hand<F>(&self, predicate: F) -> Option<usize>
    where
        F: Fn(&Card) -> bool,
    {
        self.hand.iter().position(predicate)
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
    
    pub fn add_card_to_exhaust(&mut self, card: Card) {
        self.exhaust_pile.push(card);
    }
    
    pub fn add_card_to_draw_pile(&mut self, card: Card) {
        self.draw_pile.push(card);
    }
    
    pub fn remove_dead_enemies(&mut self) {
        self.enemies.retain(|enemy| enemy.is_alive());
    }
    
    pub fn execute_enemy_intent(&mut self, enemy_id: usize, intent: &Intent, targets: &[EntityId]) {
        let source = EntityId::Enemy(enemy_id);
        
        intent.execute(self, source, targets, None);

        self.fire_event(GameEvent::EnemyAction {
            enemy: source,
        });
    }
    
    pub fn execute_all_enemy_turns(&mut self) {
        let enemy_count = self.enemies.len();
        
        for enemy_id in 0..enemy_count {
            if !self.enemies[enemy_id].is_alive() {
                continue;
            }
            
            self.process_enemy_turn_start(enemy_id);
            
            let intent = self.enemies[enemy_id].get_intent(self.turn_count);
            let intent_desc = intent.description().to_string();
            
            self.record_enemy_intent(enemy_id, intent_desc);
            
            self.execute_enemy_intent(enemy_id, &intent, &[EntityId::Player]);
            
            self.process_enemy_turn_end(enemy_id);
        }
        
        self.remove_dead_enemies();
        
        self.turn_count += 1;
    }
    
    pub fn start_player_turn(&mut self) {
        if self.current_turn_record.turn_number > 0 || !self.current_turn_record.cards_played.is_empty() {
            self.turn_history.push(self.current_turn_record.clone());
        }
        
        self.current_turn_record = TurnRecord::new(self.turn_count);
        
        self.apply_poison(EntityId::Player);
        self.player.decay_debuffs();
        
        if !self.player.has_modifier(&Modifier::RetainBlock) {
            self.player.set_block(0);
        }
        
        self.player.refill_energy();
        self.player.reset_hero_ability();
        
        if let Some(upgraded_card) = self.pending_upgraded_card.take() {
            self.hand.push(upgraded_card);
        }
        
        let default_card_drawn_at_start_of_turn = 5;
        self.draw_cards(default_card_drawn_at_start_of_turn);
        
        self.fire_event(GameEvent::TurnStarted { entity: EntityId::Player });
    }
    
    pub fn process_enemy_turn_start(&mut self, enemy_id: usize) {
        self.apply_poison(EntityId::Enemy(enemy_id));
        
        if let Some(enemy) = self.enemies.get_mut(enemy_id) {
            if !enemy.has_modifier(&Modifier::RetainBlock) {
                enemy.set_block(0);
            }
        }
    }
    
    pub fn process_enemy_turn_end(&mut self, enemy_id: usize) {
        if let Some(enemy) = self.enemies.get_mut(enemy_id) {
            enemy.decay_debuffs();
        }
    }
    
    fn apply_poison(&mut self, entity: EntityId) {
        let poison = self.get_status(entity, StatusType::Poison);
        if poison > 0 {
            self.modify_hp(entity, -poison);
            
            match entity {
                EntityId::Player => {
                    self.player.reduce_status(StatusType::Poison, 1);
                }
                EntityId::Enemy(id) => {
                    if let Some(enemy) = self.enemies.get_mut(id) {
                        enemy.reduce_status(StatusType::Poison, 1);
                    }
                }
            }
        }
    }
    
    pub fn end_player_turn(&mut self) {
        self.fire_event(GameEvent::TurnEnded { entity: EntityId::Player });
        
        if !self.player.has_modifier(&Modifier::RetainHand) {
            self.discard_hand();
        }
    }
    
    pub fn start_enemy_phase(&mut self) {
        let enemy_count = self.enemies.len();
        for enemy_id in 0..enemy_count {
            self.process_enemy_turn_start(enemy_id);
        }
    }
    
    pub fn get_turn_count(&self) -> usize {
        self.turn_count
    }
    
    pub fn is_player_dead(&self) -> bool {
        !self.player.is_alive()
    }
    
    pub fn are_all_enemies_dead(&self) -> bool {
        self.enemies.iter().all(|e| !e.is_alive())
    }
    
    pub fn is_combat_over(&self) -> bool {
        self.is_player_dead() || self.are_all_enemies_dead()
    }
    
    pub fn living_enemy_count(&self) -> usize {
        self.enemies.iter().filter(|e| e.is_alive()).count()
    }
    
    pub fn use_hero_ability(&mut self) -> Result<(), String> {
        const HERO_ABILITY_COST: i32 = 1;
        const HERO_ABILITY_DAMAGE: i32 = 2;
        
        if self.player.hero_ability_used() {
            return Err("Hero ability already used this turn".to_string());
        }
        
        if self.player.get_energy() < HERO_ABILITY_COST {
            return Err(format!("Not enough energy: need {}, have {}", 
                HERO_ABILITY_COST, self.player.get_energy()));
        }
        
        if self.hand.is_empty() {
            return Err("No cards in hand to exhaust".to_string());
        }
        
        self.player.spend_energy(HERO_ABILITY_COST);
        self.player.use_hero_ability();
        
        let card_index = self.hand.len() - 1;
        let card = self.hand.remove(card_index);
        
        let upgraded_card = (card.upgrade_fn())(card.instance_id());
        self.pending_upgraded_card = Some(upgraded_card);
        
        self.exhaust_pile.push(card);
        
        self.modify_hp(EntityId::Player, -HERO_ABILITY_DAMAGE);
        
        Ok(())
    }
}

