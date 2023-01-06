use core::ops::{Generator, GeneratorState};
use core::iter::Iterator;
use core::marker::Unpin;
use core::pin::Pin;

/// an iterator that holds an internal generator representing
/// the iteration state
#[derive(Copy, Clone, Debug)]
pub struct GenIter<T>(pub T)
where
    T: Generator<Return = ()> + Unpin;

impl<T> Iterator for GenIter<T>
where
    T: Generator<Return = ()> + Unpin,
{
    type Item = T::Yield;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.0).resume(()) {
            GeneratorState::Yielded(n) => Some(n),
            GeneratorState::Complete(()) => None,
        }
    }
}

impl<G> From<G> for GenIter<G>
where
    G: Generator<Return = ()> + Unpin,
{
    #[inline]
    fn from(gen: G) -> Self {
        GenIter(gen)
    }
}


/// macro to simplify iterator - via - generator construction
///
/// ```
/// #![feature(generators)]
///
/// use gen_iter::gen_iter;
///
/// let mut g = gen_iter!({
///     yield 1;
///     yield 2;
/// });
///
/// assert_eq!(g.next(), Some(1));
/// assert_eq!(g.next(), Some(2));
/// assert_eq!(g.next(), None);
///
/// ```
#[macro_export]
macro_rules! gen_iter {
    ($block: block) => {
        $crate::GenIter(|| $block)
    };
    (move $block: block) => {
        $crate::GenIter(move || $block)
    }
}


#[cfg(test)]
mod tests {
    use super::GenIter;

    #[test]
    fn it_works() {
        let mut g = gen_iter!({
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
        let mut g = gen_iter!(move {
            yield 1;
            yield 2;
        });

        assert_eq!(g.next(), Some(1));
        assert_eq!(g.next(), Some(2));
        assert_eq!(g.next(), None);
    }
}
