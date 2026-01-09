use crate::mechanics::base_state::{BaseState, State};
#[derive(Clone, Debug)]
pub struct Player {
    base_state: BaseState,
    max_energy: i32,
    energy: i32,
}

impl Player {
    pub fn new(name: String, max_health: i32) -> Self {
        Player {
            base_state: BaseState::new(name, max_health),
            max_energy: 3,
            energy: 3,
        }
    }
    
    pub fn base_state(&self) -> &BaseState {
        &self.base_state
    }

    pub fn base_state_mut(&mut self) -> &mut BaseState {
        &mut self.base_state
    }
}

impl State for Player {
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
    fn get_vulnerable(&self) -> i32 {
        self.base_state.get_vulnerable()
    }
    fn get_weak(&self) -> i32 {
        self.base_state.get_weak()
    }
    fn is_alive(&self) -> bool {
        self.base_state.is_alive()
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
    fn apply_vulnerable(&mut self, turns: i32) {
        self.base_state.apply_vulnerable(turns)
    }
    fn apply_weak(&mut self, turns: i32) {
        self.base_state.apply_weak(turns)
    }
    fn end_turn(&mut self) {
        self.base_state.end_turn()
    }
    fn start_turn(&mut self, armor_fn: Option<&dyn Fn(i32) -> i32>) {
        self.base_state.start_turn(armor_fn)
    }
}
