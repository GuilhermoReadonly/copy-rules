#[macro_use]
extern crate log;
extern crate log4rs;
extern crate copy_rules;

use copy_rules::configuration::Configuration;
use std::env;
use std::process;


fn main() {

    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        println!("Error while reading log4rs.yml: {}", e);
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    let mut properties_file: &str = "./config.json";

    if args.len() > 1 {
        info!("The first argument is {}", args[1]);
        properties_file = &args[1];
    }

    let config: String = match std::fs::read_to_string(properties_file) {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while reading config file: {:?}", error);
            process::exit(1);
        },
    };

    let config: Configuration = match serde_json::from_str(&config) {
        Ok(f) => f,
        Err(error) => {
            error!("An error occured while deserializing config: {:?}", error);
            process::exit(1);
        },
    };

    info!("Start jobs");
    if let Err(e) = copy_rules::run(config) {
        error!("Application error: {}", e);
        process::exit(1);
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
