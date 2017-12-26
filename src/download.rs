use std::{ env, io, fs };
use std::io::Read;
use reqwest;

pub struct Download(reqwest::header::Cookie);

impl Download {
    pub fn new() -> Download {
        let session = env::var("SESSION")
            .map(|s| {
                println!("Reading session cookie from SESSION environment variable");
                s
            })
            .or_else::<(env::VarError, io::Error), _>(|e1| {
                fs::File::open("SESSION")
                    .and_then(|mut f| {
                        let mut contents = String::new();
                        f.read_to_string(&mut contents)?;

                        println!("Reading session cookie from SESSION file");
                        Ok(contents)
                    })
                    .map_err(|e2| (e1, e2))
            })
            .expect("SESSION environment variable or file is required (your session cookie)");

        let mut cookie = reqwest::header::Cookie::new();
        cookie.append("session", session);

        Download(cookie)
    }

    pub fn input(&self, year: usize, day: usize) -> String {
        let url = format!("http://adventofcode.com/{}/day/{}/input", year, day);

        let client = reqwest::Client::new();
        let mut res = client.get(&url)
            .header(self.0.clone())
            .send()
            .expect("Error requesting input");

        assert!(res.status().is_success(), "Error requesting input (invalid session cookie?)");

        res.text().expect("Error reading input")
    }

    pub fn single_input(&self, year: usize, day: usize) -> String {
        let mut in_str = self.input(year, day);
        in_str.pop();

        in_str
    }
}
