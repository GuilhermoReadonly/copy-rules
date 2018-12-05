#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use configuration::Configuration;
use configuration::Job;
use std::error::Error;

mod fs;
mod api;
pub mod configuration;


pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {

    info!("Configuration : {:#?}",config);

    for job in &config.jobs{
        match job {
            Job::Copy(job_copy) => {
                info!("Will treat {}", job_copy.name);
                let nb_bytes = fs::copy(&job_copy.dir_from, &job_copy.dir_to)?;
                info!("Copied successfully {} bytes !", nb_bytes);
            },
            Job::RestCall(job_rest_call) => {
                info!("Will treat {}", job_rest_call.name);
                let result = api::call_verb_on_url(&job_rest_call.verb, &job_rest_call.url)?;
                info!("Result {} for calling {} in {:?}", result.status(), result.url(), &job_rest_call.verb);
            },
        };
    }

    Ok(())
}
