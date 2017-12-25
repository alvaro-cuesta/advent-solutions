use std::cmp;

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
