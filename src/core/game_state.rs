use crate::core::{Player, card::Card};
use crate::core::base_state::StatusType;
use crate::core::enemy::BaseEnemy;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardId(pub u32);

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
    CardPlayed { card: CardId, source: EntityId },
    DamageDealt { source: EntityId, target: EntityId, amount: i32 },
    BlockGained { entity: EntityId, amount: i32 },
}

pub struct GameState {
    pub player: Player,
    pub enemies: Vec<BaseEnemy>,
    pub effects: Vec<(EntityId, Box<dyn crate::core::effects::Effect>)>,
    
    card_registry: HashMap<CardId, Card>,
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
}
