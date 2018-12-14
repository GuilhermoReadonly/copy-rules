use crate::fs;
use crate::api;

pub trait Named {
    fn get_name(&self) -> &str;
}

pub trait Runable {
    fn run(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub jobs: Vec<Box<Runable>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Verb {
    DELETE,
    PUT,
    GET,
    POST,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobCopy {
    pub name: String,
    pub dir_from: String,
    pub dir_to: String,
}

impl Runable for JobCopy {
    fn run(&self) -> String {
        let nb_bytes = fs::copy(&self.dir_from, &self.dir_to);
        format!("{:?}", nb_bytes)
    }
}

impl Named for JobCopy {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobRestCall {
    pub name: String,
    pub url: String,
    pub verb: Verb,
}

impl Runable for JobRestCall {
    fn run(&self) -> String {
        let result = api::call_verb_on_url(&self.verb, &self.url);
        format!("{:?}", result)
    }
}

impl Named for JobRestCall {
    fn get_name(&self) -> &str {
        &self.name
    }
}
