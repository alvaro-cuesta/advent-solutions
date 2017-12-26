//! Solutions for [Advent of Code].
//!
//!  [Advent of Code]: http://adventofcode.com/about

#![feature(conservative_impl_trait)]
#![feature(try_from)]

extern crate itertools;
extern crate reqwest;
#[macro_use] extern crate nom;

pub mod download;
#[macro_use] pub mod parse;
pub mod iter;

mod direction;
pub use direction::Direction;

pub mod advent2017;
