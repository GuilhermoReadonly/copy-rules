extern crate fs_extra;
extern crate hyper;

use fs_extra::dir;
use fs_extra::TransitProcess;
use fs_extra::copy_items_with_progress;
use hyper::Client;
use hyper::Request;
use hyper::Method;
use hyper::Body;
use hyper::rt;
use hyper::rt::Future;

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
     // copy dir1 and file1.txt to target/dir1 and target/file1.txt
     let mut from_paths = Vec::new();
     from_paths.push(DIR_FROM);
     let res = copy_items_with_progress(&from_paths, DIR_TO, &options, handle);

     match res {
         Result::Err(e) => println!("Error !!! {}", e),
         _ => println!("Copied successfully!!!"),
     }

    println!("Start refresh rules by calling API");

    let client = Client::new();

    let uri: hyper::Uri = URL_CONNECTOR.parse().unwrap();

    let mut req = Request::new(Body::empty());
    *req.method_mut() = Method::DELETE;
    *req.uri_mut() = uri.clone();

    let response = client.request(req).map(|res| {
        println!("Response: {}", res.status());
    })
    .map_err(|err| {
        eprintln!("Error: {}", err);
    });

    rt::run(response);

    println!("All done, that's all folks...");
}
