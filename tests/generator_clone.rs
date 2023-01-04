#![feature(generators, generator_clone)]

extern crate gen_iter;
use gen_iter::*;

#[test]
fn gen_iter_clone() {
    let mut g = gen_iter!(move {
        yield 1;
        yield 2;
    });
    assert_eq!((&mut g).next(), Some(1));

    let mut g_cloned = g.clone();
    assert_eq!((&mut g_cloned).next(), Some(2));
    assert_eq!((&mut g_cloned).next(), None);

    assert_eq!((&mut g).next(), Some(2));
    assert_eq!((&mut g).next(), None);
}

    
#[test]
fn gen_iter_return_clone() {
    let mut g = gen_iter_return!(move {
        yield 1;
        yield 2;
        return "done";
    });
    assert_eq!((&mut g).next(), Some(1));

    let mut g_cloned = g.clone();
    assert_eq!((&mut g_cloned).next(), Some(2));
    assert_eq!((&mut g_cloned).next(), None);
    assert_eq!(g_cloned.return_or_self().ok(), Some("done"));

    assert_eq!((&mut g).next(), Some(2));
    assert_eq!((&mut g).next(), None);
    assert_eq!(g.return_or_self().ok(), Some("done"));
}
