//! Solutions for [Advent of Code 2017].
//!
//!  [Advent of Code 2017]: http://adventofcode.com/2017

#![feature(conservative_impl_trait)]

extern crate advent;
extern crate itertools;

#[path="1-inverse_captcha.rs"] pub mod day1;
#[path="2-corruption_checksum.rs"] pub mod day2;
#[path="3-spiral_memory.rs"] pub mod day3;
#[path="4-high_entropy_passphrases.rs"] pub mod day4;
#[path="5-maze_of_twisty_trampolines.rs"] pub mod day5;
#[path="6-memory_reallocation.rs"] pub mod day6;
