use crate::mechanics::Player;
use crate::mechanics::base_state::{BaseState, State, StatusType, Status};
use crate::mechanics::card::Deck;

#[derive(Clone, Debug)]
pub enum EnemyAction {
    Attack(i32),
    Defend(i32),
    GainStrength(i32),
    ApplyWeak(i32),
    MultiAttack(i32, i32),
    Nothing,
}

pub type EnemyIntent = Vec<EnemyAction>;

#[derive(Clone, Debug)]
pub struct BaseEnemy {
    base_state: BaseState,
    id: String,
}

impl BaseEnemy {
    pub fn new(id: String, name: String, max_health: i32) -> Self {
        BaseEnemy {
            base_state: BaseState::new(name, max_health),
            id,
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl State for BaseEnemy {
    fn get_name(&self) -> &str {
        self.base_state.get_name()
    }
    
    fn get_max_health(&self) -> i32 {
        self.base_state.get_max_health()
    }
    
    fn get_current_health(&self) -> i32 {
        self.base_state.get_current_health()
    }
    
    fn get_block(&self) -> i32 {
        self.base_state.get_block()
    }
    
    fn is_alive(&self) -> bool {
        self.base_state.is_alive()
    }
    
    fn get_status(&self, status_type: &StatusType) -> i32 {
        self.base_state.get_status(status_type)
    }
    
    fn get_all_statuses(&self) -> &Vec<Status> {
        self.base_state.get_all_statuses()
    }
    
    fn take_damage(&mut self, damage: i32) {
        self.base_state.take_damage(damage)
    }
    
    fn heal(&mut self, amount: i32) {
        self.base_state.heal(amount)
    }
    
    fn gain_block(&mut self, amount: i32) {
        self.base_state.gain_block(amount)
    }
    
    fn add_status(&mut self, status_type: StatusType, stacks: i32) {
        self.base_state.add_status(status_type, stacks)
    }
}

pub trait EnemyBehavior{
    fn get_all_possible_intents(&self) -> Vec<EnemyIntent>;

    fn get_next_intent(&self, turn: i32, player: &Player, draw_pile: &Deck, discard_pile: &Deck) -> EnemyIntent;
    
    fn execute_intent(&mut self, player: &mut Player, draw_pile: &mut Deck, discard_pile: &mut Deck);
    
    fn get_intent_description(&self) -> String;
}