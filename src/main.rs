#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use configuration::Configuration;
use configuration::Job;
use std::env;

mod fs;
mod api;
mod configuration;


fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let args: Vec<_> = env::args().collect();
    let mut properties_file: &str = "./config.json";

    if args.len() > 1 {
        info!("The first argument is {}", args[1]);
        properties_file = args[1].as_str();
    }

    let config: String = std::fs::read_to_string(properties_file).expect("An error occured while reading config file");

    let config: Configuration = serde_json::from_str(&config).expect("An error occured while deserializing config");

    info!("Configuration : {:#?}",config);

    for job in &config.jobs{
        match job {
            Job::Copy(job_copy) => {
                info!("Will treat {}", job_copy.name);
                let nb_bytes = fs::copy(&job_copy.dir_from, &job_copy.dir_to).expect(&format!("Error while treating {:?} !!!", job));
                info!("Copied successfully {} bytes !", nb_bytes);
            },
            Job::RestCall(job_rest_call) => {
                info!("Will treat {}", job_rest_call.name);
                let result = api::call_delete_on_url(&job_rest_call.url).expect(&format!("Error while treating {:?} !!!", job));
                info!("Result {} for calling {} in DELETE", result.status(), result.url());
            },
        };
    }

    info!("All done, that's all folks...");
}
