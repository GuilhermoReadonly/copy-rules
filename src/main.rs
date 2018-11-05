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

    let args: Vec<_> = env::args().collect();
    let mut properties_file: &str = "./config.json";
    
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        properties_file = args[1].as_str();
    }

    match std::fs::read_to_string(properties_file){
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
                    eprintln!("An error occured while deserializing config from file {} : {}", properties_file, e);
                },
            };
        },
        Err(e)=>{
            eprintln!("An error occured while opening file {} : {}", properties_file, e);
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
    use std::thread;
    use std::time::Duration;

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
        
        result_setup.unwrap().join().unwrap();
    }

    fn setup() -> Result<thread::JoinHandle<()>, io::Error>{
        fs::create_dir_all(TEST_DIR_1)?;
        fs::OpenOptions::new().create(true).write(true).open(TEST_FILE)?;
        fs::create_dir_all(TEST_DIR_2)?;
        
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1000));
            }
        });

        Ok(handle)
    }

    fn clean() -> Result<(), io::Error>{
        fs::remove_dir_all(TEST_DIR)?;
        Ok(())
    }

}
