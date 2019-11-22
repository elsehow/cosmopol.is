use url::Url;
use chrono::prelude::DateTime;
use chrono::prelude::Local;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, PartialEq, Deserialize)]
struct Link {
    url: String,
    title: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Collaborator {
    name: String,
    links: Vec<Link>,
}

impl fmt::Display for Collaborator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Attribute {
    name: String,
    description: Option<String>,
}

struct Press {
    url: Url,
    title: String,
    publication: String,
    project: String,
    date: DateTime<Local>,
}

struct Project {
    name: String,
    description: String,
}


struct Item {
    date: DateTime<Local>,
    url: Url,
    title: String,
    description: String,
    links: Vec<Link>,
    collaborators: Vec<Collaborator>,
    projects: Vec<Project>,
    attributes: Vec<Attribute>,
}

// fn main() {
//     println!("Hello, world!");
// }

fn main() -> Result<(), Box<std::error::Error>> {
    let f = std::fs::File::open("config/collaborators.yaml")?;
    let d: Vec<Collaborator> = serde_yaml::from_reader(f)?;
    println!("{}", d.first().unwrap());
    Ok(())
}
