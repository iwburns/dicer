extern crate dicer;
extern crate rand;

use dicer::Die;
use dicer::DieSet;
use dicer::Dice;

use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    let mut die_set = DieSet::new_with_capacity(2);
    die_set.add_dice(Die::new(6));
    die_set.add_dice(Die::new(6));

    let roll = die_set.roll(&mut rng);

    assert!(roll < 13);
    assert!(roll > 0);
}
