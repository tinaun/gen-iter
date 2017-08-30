//! # gen_iter - create generators to use as iterators
//! 
//! `GenIter` converts a generator into an iterator over the
//! yielded type of the generator. the return type of the generator
//! , if any, is ignored.
//!
//! ```
//! #![feature(generators)]
//! #![feature(conservative_impl_trait)]
//!
//! extern crate gen_iter;
//! use gen_iter::GenIter;
//!
//! fn fibonacci() -> impl Iterator<Item = u64> {
//!     GenIter(|| {
//!         let mut a = 0;
//!         let mut b = 1;
//!
//!         loop {
//!             let c = a + b;
//!             a = b;
//!             b = c;
//!             
//!             yield a;
//!         }
//!     })
//! }
//!
//! fn main() {
//!     for elem in fibonacci().map(|x| 2 * x).take(10) {
//!         println!("{}", elem);
//!     }
//! }
//! ```
//!

#![feature(generators, generator_trait)]
// #![feature(conservative_impl_trait)]

use std::ops::{Generator, GeneratorState};
use std::iter::Iterator;

/// a iterator that holds an internal generator representing
/// the iteration state
#[derive(Copy, Clone, Debug)]
pub struct GenIter<T>(pub T)
    where T: Generator;

impl<T: Generator> Iterator for GenIter<T> {
    type Item = <T as Generator>::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(n) => Some(n),
            GeneratorState::Complete(_) => None,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::GenIter;

    #[test]
    fn it_works() {
        let mut g = GenIter(|| {
            yield 1;
            yield 2;
        });

        assert_eq!(g.next(), Some(1));
        assert_eq!(g.next(), Some(2));
        assert_eq!(g.next(), None);
    }
}
