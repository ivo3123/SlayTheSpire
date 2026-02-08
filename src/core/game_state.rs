use crate::core::{Player, card::Card};
use crate::core::base_state::{StatusType, State};
use crate::core::enemy::BaseEnemy;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemyId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityId {
    Player,
    Enemy(EnemyId),
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    TurnStarted { entity: EntityId },
    TurnEnded { entity: EntityId },
    CardPlayed { card: u32, source: EntityId },
    DamageDealt { source: EntityId, target: EntityId, amount: i32 },
    BlockGained { entity: EntityId, amount: i32 },
}

pub struct GameState {
    pub player: Player,
    pub enemies: Vec<BaseEnemy>,
    pub effects: Vec<(EntityId, Box<dyn crate::core::effects::Effect>)>,
    
    card_registry: HashMap<u32, Card>,
}

impl GameState {
    pub fn new(player: Player, enemies: Vec<BaseEnemy>) -> Self {
        GameState {
            player,
            enemies,
            effects: Vec::new(),
            card_registry: HashMap::new(),
        }
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
                self.enemies.get(id.0 as usize)
                    .map(|e| e.get_status(&status_type))
                    .unwrap_or(0)
            }
        }
    }
    
    fn get_block(&self, entity: EntityId) -> i32 {
        match entity {
            EntityId::Player => self.player.get_block(),
            EntityId::Enemy(id) => {
                self.enemies.get(id.0 as usize)
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
                if let Some(enemy) = self.enemies.get_mut(id.0 as usize) {
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
                if let Some(enemy) = self.enemies.get_mut(id.0 as usize) {
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
    
    pub fn play_card(&mut self, card: &Card, source: EntityId, target: Option<EntityId>) {
        for effect in card.effects() {
            effect.resolve(self, source, target);
        }

        self.fire_event(GameEvent::CardPlayed {
            card: card.instance_id(),
            source,
        });
    }
}
