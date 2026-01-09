use crate::mechanics::base_state::State;
use crate::mechanics::enemy::{EnemyBehavior, EnemyIntent, BaseEnemy, EnemyAction};
use crate::mechanics::player::Player;
use crate::mechanics::card::Deck;

pub struct Dragonling {
    state: BaseEnemy,
}

impl Dragonling {
    pub fn new() -> Box<dyn EnemyBehavior> {
        Box::new(Dragonling {
            state: BaseEnemy::new("Dragonling".to_string(), "Dragonling".to_string(), 50),
        })
    }
}

impl EnemyBehavior for Dragonling {
    fn get_all_possible_intents(&self) -> Vec<EnemyIntent> {
        vec![
            vec![
                EnemyAction::Attack(8),
            ]
        ]
    }

    fn execute_intent(&mut self, _player: &mut Player, _draw_pile: &mut Deck, _discard_pile: &mut Deck) {
        // Implementation of intent execution
    }
    
    fn get_intent_description(&self) -> String {
        "The Dragonling prepares to attack!".to_string()
    }

    fn get_next_intent(&self, _turn: i32, _player: &Player, _draw_pile: &Deck, _discard_pile: &Deck) -> EnemyIntent {
        vec![
            EnemyAction::Attack(8),
        ]
    }
}

impl State for Dragonling {
    fn get_name(&self) -> &str {
        self.state.get_name()
    }
    fn get_max_health(&self) -> i32 {
        self.state.get_max_health()
    }
    fn get_current_health(&self) -> i32 {
        self.state.get_current_health()
    }
    fn get_block(&self) -> i32 {
        self.state.get_block()
    }
    fn get_vulnerable(&self) -> i32 {
        self.state.get_vulnerable()
    }
    fn get_weak(&self) -> i32 {
        self.state.get_weak()
    }
    fn is_alive(&self) -> bool {
        self.state.is_alive()
    }
    fn take_damage(&mut self, damage: i32) {
        self.state.take_damage(damage)
    }
    fn heal(&mut self, amount: i32) {
        self.state.heal(amount)
    }
    fn gain_block(&mut self, amount: i32) {
        self.state.gain_block(amount)
    }
    fn apply_vulnerable(&mut self, turns: i32) {
        self.state.apply_vulnerable(turns)
    }
    fn apply_weak(&mut self, turns: i32) {
        self.state.apply_weak(turns)
    }
    fn end_turn(&mut self) {
        self.state.end_turn()
    }
    fn start_turn(&mut self, armor_fn: Option<&dyn Fn(i32) -> i32>) {
        self.state.start_turn(armor_fn)
    }
}