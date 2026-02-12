use crate::core::card::{Card, CardType, Cost};
use crate::core::card::CardTargeting;
use crate::core::action::Action;
use crate::core::game_state::{GameState, EntityId};
use crate::core::effects::Effect;
use crate::core::base_state::{Modifier, State};

#[derive(Clone, Debug)]
pub struct DamageEffect {
    pub amount: i32,
}

impl Action for DamageEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, targets: &[EntityId], energy_spent: Option<i32>) {
        let damage = energy_spent.unwrap_or(self.amount);
        
        for &target_id in targets {
            game_state.deal_damage(source, target_id, damage);
        }
    }
    
    fn description(&self) -> String {
        format!("Deal {} damage", self.amount)
    }
    
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct BlockEffect {
    pub amount: i32,
}

impl Action for BlockEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _targets: &[EntityId], energy_spent: Option<i32>) {
        let block = energy_spent.unwrap_or(self.amount);
        game_state.gain_block(source, block);
    }
    
    fn description(&self) -> String {
        format!("Gain {} Block", self.amount)
    }

    fn clone_box(&self) -> Box<dyn Action> {
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

#[derive(Clone, Debug)]
pub struct AddModifierAction {
    pub modifier: Modifier,
}

impl Action for AddModifierAction {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _targets: &[EntityId], _energy_spent: Option<i32>) {
        match source {
            EntityId::Player => game_state.player_mut().add_modifier(self.modifier.clone()),
            EntityId::Enemy(id) => {
                if let Some(enemy) = game_state.enemies_mut().get_mut(id) {
                    enemy.add_modifier(self.modifier.clone());
                }
            }
        }
    }
    
    fn description(&self) -> String {
        match self.modifier {
            Modifier::RetainBlock => "Block is not removed at the start of your turn".to_string(),
            Modifier::RetainHand => "Do not discard hand at end of turn".to_string(),
        }
    }
    
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Action for ApplyEffect {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, _targets: &[EntityId], _energy_spent: Option<i32>) {
        game_state.add_effect(source, self.effect.clone_box());
    }
    
    fn description(&self) -> String {
        self.effect.ui_state().description
    }
    
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

pub fn strike(instance_id: u32) -> Card {
    Card::new(
        instance_id,
        "strike".to_string(),
        "Strike".to_string(),
        Cost::Fixed(1),
        CardType::Attack,
        CardTargeting::SingleEnemy,
        vec![Box::new(DamageEffect { amount: 6 })],
        "Deal 6 damage.".to_string(),
    )
}

pub fn defend(instance_id: u32) -> Card {
    Card::new(
        instance_id,
        "defend".to_string(),
        "Defend".to_string(),
        Cost::Fixed(1),
        CardType::Skill,
        CardTargeting::Self_,
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
        Cost::Fixed(1),
        CardType::Power,
        CardTargeting::Self_,
        vec![Box::new(ApplyEffect {
            effect: Box::new(Ritual { amount: 2 }),
        })],
        "Gain 2 Strength.".to_string(),
    )
}

pub fn barricade(instance_id: u32) -> Card {
    Card::new(
        instance_id,
        "barricade".to_string(),
        "Barricade".to_string(),
        Cost::Fixed(3),
        CardType::Power,
        CardTargeting::None,
        vec![Box::new(AddModifierAction {
            modifier: Modifier::RetainBlock,
        })],
        "Block is not removed at the start of your turn.".to_string(),
    )
}
