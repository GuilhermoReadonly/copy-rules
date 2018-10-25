extern crate fs_extra;

pub fn copy(dir_from: &str, dir_to: &str) -> Result<u64, fs_extra::error::Error> {
    println!("Start copy");
    println!("From folder {}", dir_from);
    println!("To folder {}", dir_to);

    let mut options = fs_extra::dir::CopyOptions::new(); //Initialize default values for CopyOptions
    options.overwrite = true;
    let handle = |_process_info: fs_extra::TransitProcess| {
        //println!("bytes copied : {} / {}", process_info.copied_bytes, process_info.total_bytes);
        fs_extra::dir::TransitProcessResult::ContinueOrAbort
    };

    let from_paths = vec![dir_from];
    fs_extra::copy_items_with_progress(&from_paths, dir_to, &options, handle)


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_no_from_path(){
        let result = copy("/this/is/a/fake/path",".");

        assert!(result.is_err(), "result : {:?}" ,result);
    }
   
}


