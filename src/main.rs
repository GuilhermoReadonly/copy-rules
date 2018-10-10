extern crate fs_extra;
extern crate reqwest;

use fs_extra::dir;
use fs_extra::TransitProcess;
use fs_extra::copy_items_with_progress;


fn main() {
    println!("Start copy");

    const DIR_FROM: &str = "/mnt/c/Users/gra/IdeaProjects/SCPAS/services/rules/rule-config/rules-config-connector/src/main/resources/rules/";
    const DIR_TO: &str = "/mnt/c/Users/gra/IdeaProjects/SCPAS/docker/compose/target/demo/deliverables/volumes/connector/";
    const URL_CONNECTOR: &str = "http://localhost:10089/rules-config-server/rules/cache";

    println!("From folder {}", DIR_FROM);
    println!("To folder {}", DIR_TO);

    let mut options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
    options.overwrite = true;
    let handle = |process_info: TransitProcess| {
        println!("bytes copied : {} / {}", process_info.copied_bytes, process_info.total_bytes);
        fs_extra::dir::TransitProcessResult::ContinueOrAbort
    };
     let from_paths = vec![DIR_FROM];
     let res = copy_items_with_progress(&from_paths, DIR_TO, &options, handle);

     match res {
         Err(e) => println!("Error !!! {}", e),
         Ok(s) => println!("Copied successfully {} bytes !", s),
     }

    println!("Start refresh rules by calling API");

    let client = reqwest::Client::new();
    let res = client.delete(URL_CONNECTOR).send();

    match  res {
        Ok(s) =>
        {
            println!("Result {} for calling {}", s.status(), s.url());
        },
        Err(e) => eprintln!("Error : {}", e.to_string()),
    }

    println!("All done, that's all folks...");
}
