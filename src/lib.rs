//! # gen_iter - create generators to use as iterators
//!
//! `GenIter` converts a generator into an iterator over the
//! yielded type of the generator. The return type of the generator needs to be `()`.
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
where
    T: Generator<Return = ()>;

impl<T> Iterator for GenIter<T>
where
    T: Generator<Return = ()>,
{
    type Item = T::Yield;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(n) => Some(n),
            GeneratorState::Complete(()) => None,
        }
    }
}

impl<G> From<G> for GenIter<G>
where
    G: Generator<Return = ()>,
{
    #[inline]
    fn from(gen: G) -> Self {
        GenIter(gen)
    }
}


/// macro to simplify iterator - via - generator construction
///
/// ```
/// # #![feature(generators)];
/// # #[macro_use]
/// # extern crate gen_iter;
/// # fn main() {
/// let mut g = gen_iter! {
///     yield 1;
///     yield 2;
/// };
///
///
/// assert_eq!(g.next(), Some(1));
/// assert_eq!(g.next(), Some(2));
/// assert_eq!(g.next(), None);
///
///
/// # }
/// ```
#[macro_export]
macro_rules! gen_iter {
    ($($exp:tt)*) => {
        $crate::GenIter(|| {
            $($exp)*
        })
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

    #[test]
    fn into_gen_iter() {
        let mut g: GenIter<_> = (|| {
            yield 1;
            yield 2;
        }).into();

        assert_eq!(g.next(), Some(1));
        assert_eq!(g.next(), Some(2));
        assert_eq!(g.next(), None);
    }

    #[test]
    fn gen_iter_macro() {
        let mut g = gen_iter!{
            yield 1;
            yield 2;
        };

        assert_eq!(g.next(), Some(1));
        assert_eq!(g.next(), Some(2));
        assert_eq!(g.next(), None);
    }
}
