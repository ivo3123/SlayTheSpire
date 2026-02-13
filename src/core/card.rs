use crate::core::action::Action;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardType {
    Attack,
    Skill,
    Power,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardTargeting {
    SingleEnemy,
    AllEnemies,
    Self_,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cost {
    Fixed(i32),
    X,  // remaining energy when played
    Free,
    Unplayable,
}

#[derive(Debug)]
pub struct Card {
    instance_id: u32,
    id: String,
    name: String,
    base_cost: Cost,
    cost_reduction: i32,
    card_type: CardType,
    targeting: CardTargeting,
    effects: Vec<Box<dyn Action>>,
    description: String,
    upgraded: bool,
    upgrade_fn: fn(u32) -> Card,
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            instance_id: self.instance_id,
            id: self.id.clone(),
            name: self.name.clone(),
            base_cost: self.base_cost.clone(),
            cost_reduction: self.cost_reduction,
            card_type: self.card_type,
            targeting: self.targeting,
            effects: self.effects.iter().map(|e| e.clone_box()).collect(),
            description: self.description.clone(),
            upgraded: self.upgraded,
            upgrade_fn: self.upgrade_fn,
        }
    }
}

impl Card {
    pub fn new(
        instance_id: u32,
        id: String,
        name: String,
        base_cost: Cost,
        card_type: CardType,
        targeting: CardTargeting,
        effects: Vec<Box<dyn Action>>,
        description: String,
        upgraded: bool,
        upgrade_fn: fn(u32) -> Card,
    ) -> Self {
        Card {
            instance_id,
            id,
            name,
            base_cost,
            cost_reduction: 0,
            card_type,
            targeting,
            effects,
            description,
            upgraded,
            upgrade_fn,
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
    
    pub fn base_cost(&self) -> &Cost {
        &self.base_cost
    }
    
    pub fn get_current_cost(&self) -> Result<Option<i32>, String> {
        match &self.base_cost {
            Cost::Fixed(base) => {
                let cost = (*base - self.cost_reduction).max(0);
                Ok(Some(cost))
            }
            Cost::X => Ok(None),
            Cost::Free => Ok(Some(0)),
            Cost::Unplayable => Err("Card is unplayable".to_string()),
        }
    }
    
    pub fn reduce_cost(&mut self, amount: i32) {
        self.cost_reduction += amount;
    }
    
    pub fn reset_cost_reduction(&mut self) {
        self.cost_reduction = 0;
    }
    
    pub fn cost_reduction(&self) -> i32 {
        self.cost_reduction
    }
    
    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }
    
    pub fn targeting(&self) -> CardTargeting {
        self.targeting
    }
    
    pub fn effects(&self) -> &Vec<Box<dyn Action>> {
        &self.effects
    }
    
    pub fn description(&self) -> &str {
        &self.description
    }
    
    pub fn is_upgraded(&self) -> bool {
        self.upgraded
    }
    
    pub fn upgrade_fn(&self) -> fn(u32) -> Card {
        self.upgrade_fn
    }
}
