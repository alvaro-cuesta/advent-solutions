use reqwest::Client;
use std::io::Read;
use std::{env, fs, io};

pub struct Downloader {
    client: Client,
    session: String,
}

impl Downloader {
    pub fn new() -> Downloader {
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

        Downloader {
            client: Client::new(),
            session,
        }
    }

    pub fn input(&self, year: usize, day: usize) -> String {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let mut res = self
            .client
            .get(&url)
            .header("Cookie", format!("session={}", self.session))
            .send()
            .expect("Error requesting input");

        assert!(
            res.status().is_success(),
            "Error requesting input (invalid session cookie?)"
        );

        res.text().expect("Error reading input")
    }
}
