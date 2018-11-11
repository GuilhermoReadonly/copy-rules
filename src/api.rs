extern crate reqwest;

pub fn call_delete_on_url(url: &str) ->  Result<reqwest::Response,reqwest::Error > {
    debug!("Refresh rules by calling API");

    let client = reqwest::Client::new();
    client.delete(url).send()
}
