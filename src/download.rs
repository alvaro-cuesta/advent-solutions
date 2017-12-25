use std::{ env, io, fs };
use std::io::Read;
use reqwest;

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

pub fn input(year: usize, day: usize) -> String {
    let url = format!("http://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::Client::new();
    let mut res = client.get(&url)
        .header(new_cookie())
        .send()
        .expect("Error requesting input");

    assert!(res.status().is_success(), "Error requesting input (invalid session cookie?)");

    res.text().expect("Error reading input")
}

pub fn single_input(year: usize, day: usize) -> String {
    let mut in_str = input(year, day);
    in_str.pop();

    in_str
}