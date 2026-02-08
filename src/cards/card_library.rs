use crate::core::card::{Card, CardType, CardEffect};
use crate::core::game_state::{GameState, EntityId};
use crate::core::effects::Effect;

#[derive(Clone, Debug)]
pub struct DamageEffect {
    pub amount: i32,
}

impl CardEffect for DamageEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, target: Option<EntityId>) {
        if let Some(target_id) = target {
            game_state.deal_damage(source, target_id, self.amount);
        }
    }
    
    fn description(&self) -> String {
        format!("Deal {} damage", self.amount)
    }
    
    fn clone_box(&self) -> Box<dyn CardEffect> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct BlockEffect {
    pub amount: i32,
}

impl CardEffect for BlockEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _target: Option<EntityId>) {
        game_state.gain_block(source, self.amount);
    }
    
    fn description(&self) -> String {
        format!("Gain {} Block", self.amount)
    }

    fn clone_box(&self) -> Box<dyn CardEffect> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct ApplyEffect {
    effect: Box<dyn Effect>,
}

impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for ApplyEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplyEffect").finish()
    }
}

impl CardEffect for ApplyEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _target: Option<EntityId>) {
        game_state.add_effect(source, self.effect.clone_box());
    }
    
    fn description(&self) -> String {
        self.effect.ui_state().description
    }
    
    fn clone_box(&self) -> Box<dyn CardEffect> {
        Box::new(self.clone())
    }
}

pub fn strike(instance_id: u32) -> Card {
    Card::new(
        instance_id,
        "strike".to_string(),
        "Strike".to_string(),
        1,
        CardType::Attack,
        vec![Box::new(DamageEffect { amount: 6 })],
        "Deal 6 damage.".to_string(),
    )
}

pub fn defend(instance_id: u32) -> Card {
    Card::new(
        instance_id,
        "defend".to_string(),
        "Defend".to_string(),
        1,
        CardType::Skill,
        vec![Box::new(BlockEffect { amount: 5 })],
        "Gain 5 Block.".to_string(),
    )
}

pub fn inflame(instance_id: u32) -> Card {
    use crate::cards::Ritual;
    
    Card::new(
        instance_id,
        "inflame".to_string(),
        "Inflame".to_string(),
        1,
        CardType::Power,
        vec![Box::new(ApplyEffect {
            effect: Box::new(Ritual { amount: 2 }),
        })],
        "Gain 2 Strength.".to_string(),
    )
}
