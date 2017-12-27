use std::{ env, io, fs };
use std::io::Read;
use reqwest::Client;
use reqwest::header::Cookie;

pub struct Downloader {
    client: Client,
    cookie: Cookie,
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

        let mut cookie = Cookie::new();
        cookie.append("session", session);

        Downloader { client: Client::new(), cookie }
    }

    pub fn input(&self, year: usize, day: usize) -> String {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let mut res = self.client
            .get(&url)
            .header(self.cookie.clone())
            .send()
            .expect("Error requesting input");

        assert!(res.status().is_success(), "Error requesting input (invalid session cookie?)");

        res.text().expect("Error reading input")
    }
}
