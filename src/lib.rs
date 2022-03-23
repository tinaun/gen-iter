//! # gen_iter - create generators to use as iterators
//!
//! `GenIter` converts a generator into an iterator over the
//! yielded type of the generator. The return type of the generator needs to be `()`.
//!
//! ```
//! #![feature(generators)]
//! #![feature(conservative_impl_trait)]
//!
//! use gen_iter::gen_iter;
//!
//! fn fibonacci() -> impl Iterator<Item = u64> {
//!     gen_iter!({
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
//! for elem in fibonacci().map(|x| 2 * x).take(10) {
//!     println!("{}", elem);
//! }
//! ```
//! 
//! `GenIterReturn` can be converted from a `Generator<()>`,
//! `&mut GenIterReturn` can be used as iterator.
//! The return value of the generator can be got after the iterator is done.
//! 
//! ```
//! #![feature(generators)]
//!
//! use gen_iter::gen_iter_return;
//!
//! let mut g = gen_iter_return!({
//!     yield 1;
//!     yield 2;
//!     return "done";
//! });
//! 
//! for y in &mut g {
//!     println!("yield {}", y);
//! }
//! println!("generator is_done={}", g.is_done()); // true
//! println!("generator returns {}", g.return_or_self().ok().unwrap()); // "done"
//! ```

#![no_std]
#![feature(generators, generator_trait)]
// #![feature(conservative_impl_trait)]

mod gen_iter;
pub use gen_iter::*;

mod gen_iter_return;
pub use gen_iter_return::*;
