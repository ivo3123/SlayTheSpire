
#[derive(Clone, Debug)]
pub struct BaseState {
    name: String,
    max_health: i32,
    current_health: i32,
    block: i32,
    strength: i32,
    vulnerable: i32,
    weak: i32,
    poison: i32,
}

pub trait State {
    fn get_name(&self) -> &str;
    fn get_max_health(&self) -> i32;
    fn get_current_health(&self) -> i32;
    fn get_block(&self) -> i32;
    fn get_vulnerable(&self) -> i32;
    fn get_weak(&self) -> i32;
    fn is_alive(&self) -> bool;
    fn take_damage(&mut self, damage: i32);
    fn heal(&mut self, amount: i32);
    fn gain_block(&mut self, amount: i32);
    fn apply_vulnerable(&mut self, turns: i32);
    fn apply_weak(&mut self, turns: i32);
    fn start_turn(&mut self, armor_fn: Option<&dyn Fn(i32) -> i32>);
    fn end_turn(&mut self);
}

impl BaseState {
    pub fn new(name: String, max_health: i32) -> Self {
        BaseState {
            name,
            max_health,
            current_health: max_health,
            block: 0,
            strength: 0,
            vulnerable: 0,
            weak: 0,
            poison: 0,
        }
    }
}

impl State for BaseState {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_max_health(&self) -> i32 {
        self.max_health
    }
    fn get_current_health(&self) -> i32 {
        self.current_health
    }
    fn get_block(&self) -> i32 {
        self.block
    }
    fn get_vulnerable(&self) -> i32 {
        self.vulnerable
    }
    fn get_weak(&self) -> i32 {
        self.weak
    }
    fn is_alive(&self) -> bool {
        self.current_health > 0
    }
    fn take_damage(&mut self, damage: i32) {
        let final_damage = if self.vulnerable > 0 {
            (damage as f32 * 1.5) as i32
        } else {
            damage
        };
        let actual_damage = (final_damage - self.block).max(0);
        self.block = (self.block - final_damage).max(0);
        self.current_health -= actual_damage;
    }
    fn heal(&mut self, amount: i32) {
        self.current_health = (self.current_health + amount).min(self.max_health);
    }
    fn gain_block(&mut self, amount: i32) {
        self.block += amount;
    }
    fn apply_vulnerable(&mut self, turns: i32) {
        self.vulnerable += turns;
    }
    fn apply_weak(&mut self, turns: i32) {
        self.weak += turns;
    }
    fn start_turn(&mut self, armor_fn: Option<&dyn Fn(i32) -> i32>) {
        let armor_fn = armor_fn.unwrap_or(&|_x| 0);
        self.block = armor_fn(self.block);
        let decrement = |x: &mut i32| (*x - 1).max(0);
        decrement(&mut self.vulnerable);
        decrement(&mut self.weak);
    }
    fn end_turn(&mut self) {}
}