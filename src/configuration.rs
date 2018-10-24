extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub targets: Vec<Target>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub name: String,
    pub dir_from: String,
    pub dir_to: String,
    pub url: String,
}
