extern crate reqwest;

use configuration::Verb;

pub fn call_verb_on_url(verb: &Verb, url: &str) ->  Result<reqwest::Response, String> {
    debug!("Calling API on {} with {:?}", url, verb);

    let client = reqwest::Client::new();

    match verb{
        Verb::DELETE => {
            Ok(client.delete(url).send().unwrap())
        },
        Verb::PUT => {
            Ok(client.put(url).send().unwrap())
        },
        _ => {
            Err(format!("This verb \"{:?}\" is still not supported", verb))
        },
    }
}
