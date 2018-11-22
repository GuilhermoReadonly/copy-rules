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
                let result = api::call_verb_on_url(&job_rest_call.verb, &job_rest_call.url).expect(&format!("Error while treating {:?} !!!", job));
                info!("Result {} for calling {} in {:?}", result.status(), result.url(), &job_rest_call.verb);
            },
        };
    }

    info!("All done, that's all folks...");

}



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    use std::io;

    const TEST_DIR: &str = "./test/";
    const TEST_DIR_1: &str = "./test/test_dir_1/";
    const TEST_DIR_2: &str = "./test/test_dir_2/";
    const TEST_DIR_RESULT: &str = "./test/test_dir_2/test_dir_1/";
    const TEST_FILE: &str = "./test/test_dir_1/test.txt";
    const TEST_FILE_RESULT: &str = "./test/test_dir_2/test_dir_1/test.txt";

    #[test]
    fn test_with_basic_config() {
        let result_setup = setup();
        assert!(result_setup.is_ok(), "##### {:#?}", result_setup);

        main();
        assert!(Path::new(TEST_DIR_RESULT).exists());
        assert!(Path::new(TEST_FILE_RESULT).exists());

        let result_clean = clean();
        assert!(result_clean.is_ok());
    }

    fn setup() -> Result<(), io::Error>{
        fs::create_dir_all(TEST_DIR_1)?;
        fs::OpenOptions::new().create(true).write(true).open(TEST_FILE)?;
        fs::create_dir_all(TEST_DIR_2)?;
        Ok(())
    }

    fn clean() -> Result<(), io::Error>{
        fs::remove_dir_all(TEST_DIR)?;
        Ok(())
    }

}
