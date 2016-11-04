extern crate rand;

use rand::Rng;
use rand::distributions::range::Range;
use rand::distributions::IndependentSample;

pub trait Dice<R: Rng> {
    fn roll(&self, &mut R) -> u32;
}

pub struct Die {
    range: Range<u32>
}

impl Die {
    pub fn new(num_faces: u32) -> Die {
        // `Range`s sample on the range [low, high)
        Die {
            range: Range::new(1, num_faces + 1)
        }
    }
}

impl<R> Dice<R> for Die where R: Rng {
    fn roll(&self, rng: &mut R) -> u32 {
        self.range.ind_sample(rng)
    }
}

pub struct DieSet<'a, R> {
    dice: Vec<Box<Dice<R> + 'a>>
}

impl<'a, R> DieSet<'a, R> {
    pub fn new(num_dice: u32, faces_per_die: u32) -> DieSet<'a, R> where R: Rng {
        let mut dice = Vec::new();

        for _ in 0..num_dice {
            dice.push(Box::new(Die::new(faces_per_die)) as Box<Dice<R>>);
        }

        DieSet {
            dice: dice
        }
    }
}

impl<'a, R> Dice<R> for DieSet<'a, R> where R: Rng {
    fn roll(&self, rng: &mut R) -> u32 {
        self.dice.iter().fold(0, |acc, ref die| acc + die.roll(rng))
    }
}
