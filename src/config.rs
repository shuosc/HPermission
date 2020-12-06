use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Checker {
    Bool(bool),
    URLSegment { url_segment: usize },
    Query { query: String },
}

impl Checker {
    fn check(&self, username: Option<&String>, path: &str, query_str: &str) -> bool {
        match self {
            Checker::Bool(result) => *result,
            Checker::URLSegment { url_segment } => {
                username.is_some()
                    && path
                        .split('/')
                        .nth(*url_segment)
                        .map(|it| it == username.unwrap())
                        .unwrap_or(false)
            }
            Checker::Query { query } => {
                username.is_some()
                    && query_str
                        .split('&')
                        .find(|x| x.starts_with(query))
                        .and_then(|it| it.split('=').nth(1))
                        .map(|it| it == username.unwrap())
                        .unwrap_or(false)
            }
        }
    }
}

fn allow() -> Checker {
    Checker::Bool(true)
}

fn disallow() -> Checker {
    Checker::Bool(false)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckerGroup {
    #[serde(default = "allow")]
    superuser: Checker,
    #[serde(default = "disallow")]
    authed: Checker,
    #[serde(default = "disallow")]
    everybody: Checker,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct URLConfig {
    prefix: String,
    redirect_location: Option<String>,
    read: CheckerGroup,
    write: CheckerGroup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    superuser: String,
    urls: Vec<URLConfig>,
}

impl Config {
    pub fn check(
        &self,
        username: Option<&String>,
        path: &str,
        query_str: &str,
        write: bool,
    ) -> (bool, Option<&String>) {
        for config in &self.urls {
            if path.starts_with(&config.prefix) {
                let check_group = if write { &config.write } else { &config.read };
                return (
                    check_group.everybody.check(username, path, query_str)
                        || check_group.authed.check(username, path, query_str)
                        || (username.map(|it| it == &self.superuser).unwrap_or(false)
                            && check_group.superuser.check(username, path, query_str)),
                    config.redirect_location.as_ref(),
                );
            }
        }
        (false, None)
    }
}

lazy_static! {
    pub static ref CONFIG: HashMap<String, Config> = {
        let mut file = std::fs::File::open("./config.toml").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    };
}
