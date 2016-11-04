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
