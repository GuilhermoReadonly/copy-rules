extern crate reqwest;

use configuration::Verb;
use std::error::Error;
use std::fmt;

pub fn call_verb_on_url(verb: &Verb, url: &str) ->  Result<reqwest::Response, Box<dyn Error>> {
    debug!("Calling API on {} with {:?}", url, verb);

    let client = reqwest::Client::new();

    match verb{
        Verb::DELETE => {
            let response = client.delete(url).send()?;
            Ok(response)
        },
        Verb::PUT => {
            let response = client.put(url).send()?;
            Ok(response)
        },
        _ => {
            Err(Box::new(VerbNotImplemented{}))
        },
    }
}

#[derive(Debug, Clone)]
struct VerbNotImplemented {}

impl Error for VerbNotImplemented {}

impl fmt::Display for VerbNotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This verb is still not supported")
    }
}
