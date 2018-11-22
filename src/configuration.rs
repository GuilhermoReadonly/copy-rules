extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Job {
    Copy(JobCopy),
    RestCall(JobRestCall),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct JobRestCall {
    pub name: String,
    pub url: String,
    pub verb: Verb,
}
