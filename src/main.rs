mod mechanics;
use mechanics::enemy::BaseEnemy;
use mechanics::player::Player;

use crate::mechanics::player;

fn main() {
    let player = Player::new("player1".to_string(), 75);
    print!("{:?}", player);
    println!("Hello, world!");
}
