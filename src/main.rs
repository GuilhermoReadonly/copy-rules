#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use configuration::Configuration;
use configuration::Job;

mod fs;
mod api;
mod configuration;

const PROPERTIES_FILE: &str = "./config.json";

fn main() {
    match std::fs::read_to_string(PROPERTIES_FILE){
        Ok(config)=>{
            match serde_json::from_str(&config){
                Ok::<Configuration,serde_json::Error>(config)=>{
                
                    println!("Configuration : {:#?}",config);
                    
                    for job in &config.jobs {
                        treat_job(job);                                 
                    }

                    println!("All done, that's all folks...");

                },
                Err(e) =>{
                    eprintln!("An error occured while deserializing config from file {} : {}", PROPERTIES_FILE, e);
                },
            };
        },
        Err(e)=>{
            eprintln!("An error occured while opening file {} : {}", PROPERTIES_FILE, e);
        },
    };
}



fn treat_job(job: &Job) {        
    match job {
        Job::Copy(job_copy) => {
            println!("Will treat {}", job_copy.name);
            match fs::copy(&job_copy.dir_from, &job_copy.dir_to) {
                Ok(s) => {
                    println!("Copied successfully {} bytes !", s);
                },
                Err(e) => {
                    eprintln!("Error while treating {:?} !!! {}", job, e);
                },
            };
        },
        Job::RestCall(job_rest_call) => {
            println!("Will treat {}", job_rest_call.name);
            match api::call_delete_on_url(&job_rest_call.url) {
                Ok(s) =>{
                    println!("Result {} for calling {} in DELETE", s.status(), s.url());
                },
                Err(e) => {
                    eprintln!("Error while treating {:?} !!! {}", job, e);
                },
            };
        },
    };                                                   
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
        assert!(result_setup.is_ok()); 
        
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

