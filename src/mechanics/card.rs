#[derive(Clone, Debug)]
pub enum CardType {
    Attack,
    Skill,
    Power,
}

#[derive(Clone, Debug)]
pub enum CardEffect {
    Damage(i32),
    DamageAllEnemies(i32),
    Block(i32),
    Draw(usize),
    GainEnergy(i32),
    Heal(i32),
    LoseHP(i32),
}

#[derive(Clone, Debug)]
pub struct Card {
    id: String,
    name: String,
    cost: i32,
    card_type: CardType,
    effects: Vec<CardEffect>,
    description: String,
}

impl Card {
    pub fn new(
        id: String,
        name: String,
        cost: i32,
        card_type: CardType,
        effects: Vec<CardEffect>,
        description: String,
    ) -> Self {
        Card {
            id,
            name,
            cost,
            card_type,
            effects,
            description,
        }
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
    pub fn effects(&self) -> &Vec<CardEffect> {
        &self.effects
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}
