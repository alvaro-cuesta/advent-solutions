extern crate reqwest;

fn new_cookie() -> reqwest::header::Cookie {
    let session = std::env::var("SESSION")
        .map(|s| {
            println!("Reading session cookie from SESSION environment variable");
            s
        })
        .or_else::<(std::env::VarError, std::io::Error, std::io::Error), _>(|e1| {
            use std::io::Read;

            std::fs::File::open("SESSION")
                .and_then(|mut f| {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)?;

                    println!("Reading session cookie from SESSION file in current directory");
                    Ok(contents)
                })
                .map_err(|e2| (e1, e2))
                .or_else(|(e1, e2)| {
                    std::fs::File::open("../SESSION")
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
    where T: std::cmp::PartialOrd,
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
          U: std::cmp::PartialOrd,
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
        use Facing::*;

        match *self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    pub fn cw(&self) -> Facing {
        use Facing::*;

        match *self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub fn reverse(&self) -> Facing {
        use Facing::*;

        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Into<(isize, isize)> for Facing {
    fn into(self) -> (isize, isize) {
        use Facing::*;

        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

impl std::ops::Add<(isize, isize)> for Facing {
    type Output = (isize, isize);

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        let (dx, dy) = self.into();
        (x + dx, y + dy)
    }
}

impl std::ops::Add<(usize, usize)> for Facing {
    type Output = (usize, usize);

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        let (dx, dy) = self.into();
        (
            (x as isize + dx) as usize,
            (y as isize + dy) as usize,
        )
    }
}

impl std::ops::Add<Facing> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, facing: Facing) -> Self::Output {
        facing + self
    }
}

impl std::ops::Add<Facing> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, facing: Facing) -> Self::Output {
        facing + self
    }
}

impl std::ops::AddAssign<Facing> for (isize, isize) {
    fn add_assign(&mut self, other: Facing) {
        *self = *self + other
    }
}

impl std::ops::AddAssign<Facing> for (usize, usize) {
    fn add_assign(&mut self, other: Facing) {
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
