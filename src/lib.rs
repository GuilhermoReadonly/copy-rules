#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use crate::configuration::Named;
use crate::configuration::Runable;
use crate::configuration::Configuration;
use std::error::Error;

mod fs;
mod api;
pub mod configuration;


pub fn run(config: Configuration) -> Result<(), Box<Error>> {

    debug!("Configuration : {:#?}",config);

    for job in &config.jobs{
        info!("Will treat {}", *job.get_name());
        let result = job.run();
        info!("Result is : {}", result);
    }

    Ok(())
}
