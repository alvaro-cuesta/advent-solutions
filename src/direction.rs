use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Direction { Up, Right, Down, Left }

use self::Direction::*;

impl Direction {
    pub fn cw(&self) -> Direction {
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn reverse(&self) -> Direction {
        match *self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn ccw(&self) -> Direction {
        match *self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}

impl<'a> Into<(isize, isize)> for &'a Direction {
    fn into(self) -> (isize, isize) {
        match *self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}

impl ops::Add<(isize, isize)> for Direction {
    type Output = (isize, isize);

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        let (dx, dy) = self.into();
        (x + dx, y + dy)
    }
}

impl<'a> ops::Add<(isize, isize)> for &'a Direction {
    type Output = (isize, isize);

    fn add(self, o: (isize, isize)) -> Self::Output {
        *self + o
    }
}

impl ops::Add<(usize, usize)> for Direction {
    type Output = (usize, usize);

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        let (dx, dy) = self.into();
        (
            (x as isize + dx) as usize,
            (y as isize + dy) as usize,
        )
    }
}

impl<'a> ops::Add<(usize, usize)> for &'a Direction {
    type Output = (usize, usize);

    fn add(self, o: (usize, usize)) -> Self::Output {
        *self + o
    }
}

impl ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, facing: Direction) -> Self::Output {
        facing + self
    }
}

impl<'a> ops::Add<&'a Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, facing: &'a Direction) -> Self::Output {
        facing + self
    }
}

impl ops::Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, facing: Direction) -> Self::Output {
        facing + self
    }
}

impl<'a> ops::Add<&'a Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, facing: &'a Direction) -> Self::Output {
        facing + self
    }
}

impl ops::AddAssign<Direction> for (isize, isize) {
    fn add_assign(&mut self, other: Direction) {
        *self = *self + other
    }
}

impl<'a> ops::AddAssign<&'a Direction> for (isize, isize) {
    fn add_assign(&mut self, other: &'a Direction) {
        *self = *self + other
    }
}

impl ops::AddAssign<Direction> for (usize, usize) {
    fn add_assign(&mut self, other: Direction) {
        *self = *self + other
    }
}

impl<'a> ops::AddAssign<&'a Direction> for (usize, usize) {
    fn add_assign(&mut self, other: &'a Direction) {
        *self = *self + other
    }
}
