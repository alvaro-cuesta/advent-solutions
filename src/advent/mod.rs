extern crate reqwest;

#[macro_use] pub mod parse;

use std::{ env, io, fs, ops, cmp };
use std::io::Read;

fn new_cookie() -> reqwest::header::Cookie {
    let session = env::var("SESSION")
        .map(|s| {
            println!("Reading session cookie from SESSION environment variable");
            s
        })
        .or_else::<(env::VarError, io::Error, io::Error), _>(|e1| {
            fs::File::open("SESSION")
                .and_then(|mut f| {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)?;

                    println!("Reading session cookie from SESSION file in current directory");
                    Ok(contents)
                })
                .map_err(|e2| (e1, e2))
                .or_else(|(e1, e2)| {
                    fs::File::open("../SESSION")
                        .and_then(|mut f| {
                            let mut contents = String::new();
                            f.read_to_string(&mut contents)?;

                            println!("Reading session cookie from SESSION file in parent directory");
                            Ok(contents)
                        })
                        .map_err(|e3| (e1, e2, e3))
                })
        })
        .expect("SESSION environment variable or file is required (your session cookie)");

    let mut cookie = reqwest::header::Cookie::new();
    cookie.append("session", session);

    cookie
}

pub fn download_input(year: usize, day: usize) -> String {
    let url = format!("http://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::Client::new();
    let mut res = client.get(&url)
        .header(new_cookie())
        .send()
        .expect("Error requesting input");

    assert!(res.status().is_success(), "Error requesting input (invalid session cookie?)");

    res.text().expect("Error reading input")
}

pub fn download_single_input(year: usize, day: usize) -> String {
    let mut input = download_input(year, day);
    input.pop();

    input
}

pub fn min_and_max<T, I>(e: I) -> Option<(T, Option<T>)>
    where T: cmp::PartialOrd,
          I: IntoIterator<Item=T>,
{
    e.into_iter()
        .fold(None, |a, x| Some(match a {
            Some((min, Some(max))) => {
                if x < min { (x, Some(max)) }
                else if x > max { (min, Some(x)) }
                else { (min, Some(max)) }
            },
            Some((both, None)) => {
                if x < both { (x, Some(both)) }
                else if x > both { (both, Some(x)) }
                else { (both, None) }
            },
            None => {
                { (x, None) }
            },
        }))
}

pub fn min_and_max_by_key<T, I, U, F>(e: I, k: F) -> Option<(T, Option<T>)>
    where I: IntoIterator<Item=T>,
          U: cmp::PartialOrd,
          F: Fn(&T) -> U,
{
    e.into_iter()
        .fold(None, |a, x| Some(match a {
            Some((min, Some(max))) => {
                if k(&x) < k(&min) { (x, Some(max)) }
                else if k(&x) > k(&max) { (min, Some(x)) }
                else { (min, Some(max)) }
            },
            Some((both, None)) => {
                if k(&x) < k(&both) { (x, Some(both)) }
                else if k(&x) > k(&both) { (both, Some(x)) }
                else { (both, None) }
            },
            None => {
                { (x, None) }
            },
        }))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Facing { Up, Down, Left, Right }

impl Facing {
    pub fn ccw(&self) -> Facing {
        use self::Facing::*;

        match *self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    pub fn cw(&self) -> Facing {
        use self::Facing::*;

        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn reverse(&self) -> Facing {
        use self::Facing::*;

        match *self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

impl Into<(isize, isize)> for Facing {
    fn into(self) -> (isize, isize) {
        use self::Facing::*;

        match self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}

impl<'a> Into<(isize, isize)> for &'a Facing {
    fn into(self) -> (isize, isize) {
        use self::Facing::*;

        match *self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}

impl ops::Add<(isize, isize)> for Facing {
    type Output = (isize, isize);

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        let (dx, dy) = self.into();
        (x + dx, y + dy)
    }
}

impl<'a> ops::Add<(isize, isize)> for &'a Facing {
    type Output = (isize, isize);

    fn add(self, o: (isize, isize)) -> Self::Output {
        *self + o
    }
}

impl ops::Add<(usize, usize)> for Facing {
    type Output = (usize, usize);

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        let (dx, dy) = self.into();
        (
            (x as isize + dx) as usize,
            (y as isize + dy) as usize,
        )
    }
}

impl<'a> ops::Add<(usize, usize)> for &'a Facing {
    type Output = (usize, usize);

    fn add(self, o: (usize, usize)) -> Self::Output {
        *self + o
    }
}

impl ops::Add<Facing> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, facing: Facing) -> Self::Output {
        facing + self
    }
}

impl<'a> ops::Add<&'a Facing> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, facing: &'a Facing) -> Self::Output {
        facing + self
    }
}

impl ops::Add<Facing> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, facing: Facing) -> Self::Output {
        facing + self
    }
}

impl<'a> ops::Add<&'a Facing> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, facing: &'a Facing) -> Self::Output {
        facing + self
    }
}

impl ops::AddAssign<Facing> for (isize, isize) {
    fn add_assign(&mut self, other: Facing) {
        *self = *self + other
    }
}

impl<'a> ops::AddAssign<&'a Facing> for (isize, isize) {
    fn add_assign(&mut self, other: &'a Facing) {
        *self = *self + other
    }
}

impl ops::AddAssign<Facing> for (usize, usize) {
    fn add_assign(&mut self, other: Facing) {
        *self = *self + other
    }
}

impl<'a> ops::AddAssign<&'a Facing> for (usize, usize) {
    fn add_assign(&mut self, other: &'a Facing) {
        *self = *self + other
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
