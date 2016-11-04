extern crate dicer;
extern crate rand;

use dicer::Die;
use dicer::Dice;

use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let roll = Die::new(6).roll(&mut rng);

    assert!(roll < 7);
    assert!(roll > 0);
}
