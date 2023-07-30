//! Implementation of J. H. Conway's surreal numbers, as explained in the book *[Surreal Numbers](https://www.amazon.com/dp/0201038129)* by Donald Knuth.

#[macro_use]
extern crate lazy_static;

mod finite;
mod infinite;

pub use finite::{div_approx, ftos, SurrealFinite};
pub use infinite::{SurrealInfinite, SurrealElement};

#[cfg(test)]
mod tests;
