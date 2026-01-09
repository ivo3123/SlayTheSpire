mod mechanics;
use mechanics::enemy::BaseEnemyState;
use mechanics::player::Player;

use crate::mechanics::player;

fn main() {
    let player = Player::new("player1".to_string(), "Hero".to_string(), 100);
    print!("{:?}", player);
    println!("Hello, world!");
}
