use crate::mechanics::base_state::BaseState;
#[derive(Clone, Debug)]
pub struct Player {
    base_state: BaseState,
    max_energy: i32,
    energy: i32,
}

impl Player {
    pub fn new(id: String, name: String, max_health: i32) -> Self {
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
