#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use configuration::Named;
use configuration::Runable;
use configuration::Configuration;
use configuration::Job;
use std::error::Error;

mod fs;
mod api;
pub mod configuration;


pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {

    debug!("Configuration : {:#?}",config);

    for job in &config.jobs{
        info!("Will treat {}", job.get_name());
        let result = job.run();
        info!("Result is : {}", result);
    }

    Ok(())
}

impl Runable for Job {
    fn run(&self) -> String {
        match &self {
            Job::Copy(job_copy) => {
                let nb_bytes = fs::copy(&job_copy.dir_from, &job_copy.dir_to);
                format!("{:?}", nb_bytes)
            },
            Job::RestCall(job_rest_call) => {
                let result = api::call_verb_on_url(&job_rest_call.verb, &job_rest_call.url);
                format!("{:?}", result)
            },
        }
    }
}
