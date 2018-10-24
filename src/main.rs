#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use configuration::Configuration;

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
                    
                    for target in &config.targets {
                        
                        match fs::copy(&target.dir_from, &target.dir_to) {
                            Err(e) => {
                                println!("Error !!! {}", e);
                                break;
                            },
                            Ok(s) => {
                                println!("Copied successfully {} bytes !", s);
                                api::call_delete_on_url(&target.url);
                            },
                        };
                        
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

