extern crate dicer;
extern crate rand;

use dicer::Die;
use dicer::DieSet;
use dicer::Dice;

use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    let mut set_a = DieSet::new();
    set_a.add_dice(Die::new(6));
    set_a.add_dice(Die::new(6));

    let mut set_b = DieSet::new();
    set_b.add_dice(set_a);
    set_b.add_dice(Die::new(6));

    let roll_b = set_b.roll(&mut rng);

    assert!(roll_b < 19);
    assert!(roll_b > 0);
}
