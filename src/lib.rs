#![doc = include_str!("../README.md")]

pub mod iter;
use iter::*;

use itertools::{Either, Itertools};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct WeightedPicker<T> {
    prob: Vec<f64>,
    alias: Vec<usize>,
    items: Vec<(T, f64)>,
}

impl<T> WeightedPicker<T> {
    /**
    Initialize a WeightedPicker from the given
    items and weights.

    Panics if you pass it an empty Vec.
    */
    #[must_use]
    pub fn new(entries: Vec<(T, f64)>) -> Self {
        assert_ne!(entries.len(), 0, "Cannot use an empty vec!");

        let total_weight: f64 = entries.iter().map(|(_, weight)| *weight).sum();
        let len = entries.len();
        let average = (len as f64).recip();

        let weights = entries.iter().map(|(_, w)| *w).collect_vec();
        let items = entries;

        let (mut small, mut large): (Vec<_>, Vec<_>) = weights
            .iter()
            .enumerate()
            .map(|(idx, weight)| {
                let prob = weight / total_weight * len as f64;
                (idx, prob)
            })
            .partition_map(|(idx, prob)| {
                // true goes to small, false to large
                if prob < average {
                    Either::Left(idx)
                } else {
                    Either::Right(idx)
                }
            });

        let mut alias = vec![0; len];
        let mut prob = vec![0.0; len];

        while !small.is_empty() && !large.is_empty() {
            // what do you mean, this is great rust code
            let less = small.pop().unwrap();
            let more = large.pop().unwrap();

            prob[less] *= len as f64;
            alias[less] = more;

            let prev_more = prob[more];
            let prev_less = prob[less];
            prob[more] = prev_more + prev_less - average;

            if prob[more] >= average {
                large.push(more)
            } else {
                small.push(more);
            }
        }
        while let Some(last) = small.pop() {
            prob[last] = 1.0;
        }
        while let Some(last) = large.pop() {
            prob[last] = 1.0;
        }

        debug_assert_eq!(prob.len(), len);
        debug_assert_eq!(alias.len(), len);
        debug_assert_eq!(items.len(), len);

        Self { alias, items, prob }
    }

    /// Randomly pick an item from the list.
    #[must_use]
    pub fn get<R: Rng + ?Sized>(&self, rng: &mut R) -> &T {
        &self.items[self.get_idx(rng)].0
    }

    /// Randomly pick an *index* from the list.
    /// This is like [`WeightedPicker::get`], but returns the index of the
    /// selected value instead of the value.
    ///
    /// You can use this function to save some space by passing a vec
    /// where `T` is `()`, if you want `usize` outputs, I guess.
    #[must_use]
    pub fn get_idx<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let column = rng.gen_range(0..self.prob.len());
        let coin_toss = rng.gen::<f64>() < self.prob[column];
        if coin_toss {
            column
        } else {
            self.alias[column]
        }
    }

    /// Get the number of entries in the picker.
    #[must_use]
    #[allow(clippy::len_without_is_empty)] // it can never be empty
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Manually index into the picker's array.
    pub fn get_by_idx(&self, idx: usize) -> Option<&T> {
        self.items.get(idx).map(|(it, _)| it)
    }

    /// Manually index into the picker's array.
    /// You can use this to mutate entries once they've been created.
    ///
    /// Note there is no way to mutate *probabilities* after creation,
    /// nor any way to add or remove possible values.
    #[must_use]
    pub fn get_mut_by_idx(&mut self, idx: usize) -> Option<&mut T> {
        self.items.get_mut(idx).map(|(it, _)| it)
    }

    /// Convenience method for creating a WeightedPicker and then calling `get`,
    /// so you don't need to actually make the WeightedPicker.
    #[must_use]
    pub fn pick<R: Rng + ?Sized>(items: Vec<(T, f64)>, rng: &mut R) -> T {
        let mut wp = WeightedPicker::new(items);
        let idx = wp.get_idx(rng);
        // this would be unsound to use after removal,
        // but fortunately we don't need to use it again
        // not sure why i can't move out of it.
        wp.items.swap_remove(idx).0
    }

    /**
    Iterate through the elements in order of probability, from least to most.

    This requires sorting internally, so is `O(n log(n))`.
    ```rust
    # use wicker::{WeightedPicker, iter::Iter};
    let picker = WeightedPicker::new(vec![
    ("hello", 0.3),
    ("wicker", 10.0),
    ("this", 3.0),
    ("world", 1.0),
    ("is", 5.0),
    ]);
    let v = picker.iter().copied().collect::<Vec<_>>();
    assert_eq!(v, &["hello", "world", "this", "is", "wicker"]);
    ```
    */
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }
}

// doctests don't println so let's replicate that test
#[test]
fn pick() {
    let picker = WeightedPicker::new(vec![
        ("common", 10.0),
        ("uncommon", 5.0),
        ("rare", 2.0),
        ("legendary", 1.0),
        ("mythic", 0.1),
    ]);

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        println!("- {}", picker.get(&mut rng));
    }
}
