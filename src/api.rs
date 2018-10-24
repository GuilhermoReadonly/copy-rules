extern crate reqwest;

pub fn call_delete_on_url(url: &str) {
    println!("Refresh rules by calling API");

    let client = reqwest::Client::new();
    let res = client.delete(url).send();
    match  res {
        Ok(s) =>{
            println!("Result {} for calling {} in DELETE", s.status(), s.url());
        },
        Err(e) => eprintln!("Error : {}", e.to_string()),
    };
}
