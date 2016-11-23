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
    pub fn new() -> DieSet<'a, R> where R: Rng {
        DieSet {
            dice: Vec::new()
        }
    }

    pub fn new_with_capacity(capacity: usize) -> DieSet<'a, R> where R: Rng {
        DieSet {
            dice: Vec::with_capacity(capacity)
        }
    }

    pub fn add_dice<D>(&mut self, dice: D) where D: Dice<R> + 'a, R: Rng {
        self.dice.push(Box::new(dice) as Box<Dice<R>>);
    }
}

impl<'a, R> Dice<R> for DieSet<'a, R> where R: Rng {
    fn roll(&self, rng: &mut R) -> u32 {
        self.dice.iter().fold(0, |acc, ref die| acc + die.roll(rng))
    }
}
