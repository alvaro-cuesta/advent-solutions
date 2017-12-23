extern crate advent;

use std::str::FromStr;
use std::ops::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum HexDirection { N, NE, SE, S, SW, NW }

impl<'a> FromStr for HexDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HexDirection::*;

        match s {
            "n" => Ok(N),
            "ne" => Ok(NE),
            "se" => Ok(SE),
            "s" => Ok(S),
            "sw" => Ok(SW),
            "nw" => Ok(NW),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct HexCoord(isize, isize);

impl HexCoord {
    fn distance(&self, other: &HexCoord) -> isize {
        let &HexCoord(aq, ar) = self;
        let &HexCoord(bq, br) = other;

        (
            (aq - bq).abs()
            + (aq + ar - bq - br).abs()
            + (ar - br).abs()
        ) / 2
    }
}

impl Add<HexDirection> for HexCoord {
    type Output = HexCoord;

    fn add(self, direction: HexDirection) -> Self::Output {
        use HexDirection::*;

        let HexCoord(q, r) = self;

        match direction {
            N => HexCoord(q, r - 1),
            NE => HexCoord(q + 1, r - 1),
            SE => HexCoord(q + 1, r),
            S => HexCoord(q, r + 1),
            SW => HexCoord(q - 1, r + 1),
            NW => HexCoord(q - 1, r),
        }
    }
}

fn main() {
    let (furthest, coord) = advent::download_single_input(2017, 11)
        .split(',')
        .map(|dir| FromStr::from_str(dir).expect("Unknown direction"))
        .fold((0, HexCoord(0, 0)), |(furthest, coord), x| {
            let new_coord = coord + x;
            let distance = new_coord.distance(&HexCoord(0, 0));

            (std::cmp::max(distance, furthest), new_coord)
        });

    println!("Step 1: {}", coord.distance(&HexCoord(0, 0)));
    println!("Step 2: {}", furthest);
}
