#![feature(generators, generator_trait)]

extern crate gen_iter;
use gen_iter::*;

use std::ops::{Generator, GeneratorState};
use std::iter::Iterator;
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::cell::RefCell;
use std::format;

/// test customize generator using `impl Generator`
/// also test `#[derive(Debug)]`
#[test]
fn gen_iter_impl_generator() {
    #[derive(Debug)]
    struct G(i32);
    impl Generator for G {
        type Yield = i32;
        type Return = ();
        fn resume(mut self: Pin<&mut Self>, _: ()) -> GeneratorState<Self::Yield, Self::Return> {
            let v = self.0;
            if v > 0 {
                self.0 = v - 1;
                GeneratorState::Yielded(v)
            } else {
                GeneratorState::Complete(())
            }
        }
    }

    let g = G(1);
    let mut g = GenIter(g);

    assert_eq!(format!("{:?}", g), "GenIter(G(1))");
    assert_eq!((&mut g).next(), Some(1));
    assert_eq!(format!("{:?}", g), "GenIter(G(0))");
    assert_eq!((&mut g).next(), None);
    assert_eq!(format!("{:?}", g), "GenIter(G(0))");
}

/// test customize generator using `impl Generator` which is `!Unpin`
/// also test `#[derive(Debug)]`
#[test]
fn gen_iter_return_impl_generator_not_unpin() {
    #[derive(Debug)]
    struct G(RefCell<i32>, PhantomPinned);
    impl G {
        pub fn new(v: i32) -> Self {
            G(RefCell::new(v), PhantomPinned)
        }
    }
    impl Generator for G {
        type Yield = i32;
        type Return = &'static str;
        fn resume(self: Pin<&mut Self>, _: ()) -> GeneratorState<Self::Yield, Self::Return> {
            let v = *self.0.borrow();
            if v > 0 {
                *self.0.borrow_mut() -= 1;
                GeneratorState::Yielded(v)
            } else {
                GeneratorState::Complete("done")
            }
        }
    }

    let mut g = G::new(1);
    let pin_g = unsafe { Pin::new_unchecked(&mut g) };
    let mut g = GenIterReturn::new(pin_g);

    assert_eq!(format!("{:?}", g), "GenIterReturn(Err(G(RefCell { value: 1 }, PhantomPinned)))");
    assert_eq!((&mut g).next(), Some(1));
    assert_eq!(g.is_complete(), false);

    g = g.try_get_return().expect_err("unexpected generator state: is_complete");
    
    assert_eq!(format!("{:?}", g), "GenIterReturn(Err(G(RefCell { value: 0 }, PhantomPinned)))");
    assert_eq!((&mut g).next(), None);
    assert_eq!(format!("{:?}", g), "GenIterReturn(Ok(\"done\"))");
    assert_eq!(g.is_complete(), true);

    assert_eq!((&mut g).next(), None); // it won't panic when call `next()` even exhausted.

    assert_eq!(g.try_get_return().ok(), Some("done"));
}
