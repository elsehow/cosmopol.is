use url::Url;
use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
struct Link {
    // #[serde(with = "url_serde")]
    // url: Url,
    url: String,
    title: String,
}

#[derive(Deserialize)]
struct Collaborator {
    name: String,
    links: Vec<Link>,
}

impl fmt::Display for Collaborator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize)]
struct Press {
    // #[serde(with = "url_serde")]
    // url: Url,
    url: String,
    title: String,
    publication: String,
    project: String,
    date: DateTime<Utc>,
}
impl fmt::Display for Press {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. {}. {}.", self.publication, self.title, self.date)
    }
}


struct Attribute {
    name: String,
    description: Option<String>,
}

struct Project {
    name: String,
    description: String,
}


struct Item {
    date: DateTime<Utc>,
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
    let collaborators: Vec<Collaborator> = serde_yaml::from_reader(f)?;
    // println!("{}", collaborators.first().unwrap());

    let f = std::fs::File::open("config/press.yaml")?;
    let press: Vec<Press> = serde_yaml::from_reader(f)?;
    println!("{}", press.first().unwrap());

    Ok(())
}
