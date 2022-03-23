use core::ops::{Generator, GeneratorState};
use core::iter::{Iterator, FusedIterator};
use core::marker::Unpin;
use core::pin::Pin;

/// `GenIterReturn<G>` is a iterator for generator with return value.
/// 
/// Differences with `GenIter<G>`:
/// 1. able to get return value of a generator
/// 2. safe to call `next()` after generator is done without panic
/// 3. maybe less efficient than `GenIter<G>`
#[derive(Copy, Clone, Debug)]
pub struct GenIterReturn<G: Generator + Unpin>(Result<G::Return, G>);

impl<G: Generator + Unpin> GenIterReturn<G> {
    #[inline]
    pub fn new(g: G) -> Self {
        GenIterReturn(Err(g))
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.0.is_ok()
    }

    #[inline]
    pub fn return_or_self(self) -> Result<G::Return, Self> {
        match self.0 {
            Ok(r) => Ok(r),
            Err(_) => Err(self),
        }
    }
}

impl<G: Generator + Unpin> Iterator for &mut GenIterReturn<G> {
    type Item = G::Yield;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Ok(_) => None,
            Err(ref mut g) => match Pin::new(g).resume(()) {
                GeneratorState::Yielded(y) => Some(y),
                GeneratorState::Complete(r) => {
                    self.0 = Ok(r);
                    None
                },
            }
        }
    }
}

impl<G: Generator + Unpin> FusedIterator for &mut GenIterReturn<G> {}

impl<G: Generator + Unpin> From<G> for GenIterReturn<G> {
    #[inline]
    fn from(g: G) -> Self {
        GenIterReturn::new(g)
    }
}

/// macro to simplify iterator - via - generator with return value construction
///
/// Examples:
/// ```
/// #![feature(generators)]
///
/// use gen_iter::gen_iter_return;
///
/// let mut g = gen_iter_return!({
///     yield 1;
///     yield 2;
///     return "done";
/// });
///
/// assert_eq!((&mut g).collect::<Vec<_>>(), [1, 2]); // use `&mut g` as an iterator
/// assert_eq!(g.is_done(), true); // check whether generator is done
/// assert_eq!((&mut g).next(), None); // safe to call `next()` after done
/// assert_eq!(g.return_or_self().ok(), Some("done")); // get return value of generator
/// ```
/// We should use `&mut g` in `for` statement, to keep `g` be valid after `for`, so we can get the return value.
/// ```compile_fail
/// #![feature(generators)]
///
/// use gen_iter::gen_iter_return;
///
/// let mut g = gen_iter_return!({
///     yield 1;
///     yield 2;
///     return "done";
/// });
/// for v in g {} // compile failed, should use `&mut g`
/// ```
#[macro_export]
macro_rules! gen_iter_return {
    ($block: block) => {
        $crate::GenIterReturn::new(|| $block)
    };
    (move $block: block) => {
        $crate::GenIterReturn::new(move || $block)
    }
}

#[cfg(test)]
mod tests {
    use super::GenIterReturn;

    #[test]
    fn it_works() {
        let mut g = gen_iter_return!({
            yield 1;
            yield 2;
            return "done";
        });

        assert_eq!((&mut g).next(), Some(1));
        assert_eq!(g.is_done(), false);
        assert_eq!((&mut g).next(), Some(2));
        assert_eq!(g.is_done(), false);
        assert_eq!((&mut g).next(), None);
        assert_eq!(g.is_done(), true);
        assert_eq!((&mut g).next(), None);
        assert_eq!(g.return_or_self().ok(), Some("done"));
    }

    #[test]
    fn gen_iter_return_from() {
        let mut g: GenIterReturn<_> = GenIterReturn::from(|| {
            yield 1;
            yield 2;
            return "done";
        });
        let mut gi = &mut g;

        assert_eq!(gi.next(), Some(1));
        assert_eq!(gi.next(), Some(2));
        assert_eq!(gi.next(), None);

        assert_eq!(g.is_done(), true);
        assert_eq!(g.return_or_self().ok(), Some("done"));
    }

    #[test]
    fn gen_iter_return_macro() {
        let mut g = gen_iter_return!({
            yield 1;
            yield 2;
            return "done";
        });

        let mut sum = 0;
        let mut count = 0;
        for y in &mut g {
            sum += y;
            count += 1;
        }
        assert_eq!(sum, 3);
        assert_eq!(count, 2);

        assert_eq!(g.is_done(), true);
        assert_eq!(g.return_or_self().ok(), Some("done"));
    }
}
