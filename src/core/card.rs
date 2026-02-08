use crate::core::game_state::{GameState, EntityId};

#[derive(Clone, Copy, Debug)]
pub enum CardType {
    Attack,
    Skill,
    Power,
}

pub trait CardEffect: std::fmt::Debug {
    fn resolve(&self, game_state: &mut GameState, source: EntityId, target: Option<EntityId>);
    fn description(&self) -> String;
    fn clone_box(&self) -> Box<dyn CardEffect>;
}

#[derive(Debug)]
pub struct Card {
    instance_id: u32,
    id: String,
    name: String,
    cost: i32,
    card_type: CardType,
    effects: Vec<Box<dyn CardEffect>>,
    description: String,
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            instance_id: self.instance_id,
            id: self.id.clone(),
            name: self.name.clone(),
            cost: self.cost,
            card_type: self.card_type,
            effects: self.effects.iter().map(|e| e.clone_box()).collect(),
            description: self.description.clone(),
        }
    }
}

impl Card {
    pub fn new(
        instance_id: u32,
        id: String,
        name: String,
        cost: i32,
        card_type: CardType,
        effects: Vec<Box<dyn CardEffect>>,
        description: String,
    ) -> Self {
        Card {
            instance_id,
            id,
            name,
            cost,
            card_type,
            effects,
            description,
        }
    }

    pub fn instance_id(&self) -> u32 {
        self.instance_id
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn cost(&self) -> i32 {
        self.cost
    }
    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }
    pub fn effects(&self) -> &Vec<Box<dyn CardEffect>> {
        &self.effects
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

pub struct Deck {
    cards: Vec<Card>,
}