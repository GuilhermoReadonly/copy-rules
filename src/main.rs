extern crate java_properties;

use std::fs::File;
use std::io::BufReader;

mod fs;
mod api;

const PROPERTIES_FILE: &str = "./copy_rules.properties";

fn main() {
    // Reading properties
    match File::open(PROPERTIES_FILE){
        Ok(f)=>{
            match java_properties::read(BufReader::new(f)){
                Ok(properties)=>{
                    println!("DEBUG {:?}",properties);

                    process_one_target(properties.get("CONNECTOR.DIR_FROM").unwrap(), properties.get("CONNECTOR.DIR_TO").unwrap(), properties.get("CONNECTOR.URL").unwrap());
                    process_one_target(properties.get("BACKEND.DIR_FROM").unwrap(), properties.get("BACKEND.DIR_TO").unwrap(), properties.get("BACKEND.URL").unwrap());

                    println!("All done, that's all folks ! <3");
                    
                },
                Err(e) =>{
                    eprintln!("An error occured while reading properties from file {} : {}", PROPERTIES_FILE, e);
                },
            };
        },
        Err(e)=>{
            eprintln!("An error occured while opening file {} : {}", PROPERTIES_FILE, e);
        },
    };

}


fn process_one_target(dir_from: &str, dir_to: &str, url: &str) {

    fs::copy(dir_from, dir_to);

    api::call_delete_on_url(url);
}
