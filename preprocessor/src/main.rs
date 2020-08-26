use std::path::*;
use std::{env, fs};

fn main() {
    let path = "C:\\Users\\eric\\dev\\CalorieCounter\\src";
    match preprocess(Path::new(path)) {
        Ok(_) => println!("Finished Preprocess"),
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn preprocess(path: &Path) -> Result<(), String> {
    let mut src_files = vec![];
    let mut files_to_process = vec![];

    let pp_file = ".pp";
    let pp_def = ".pp_def";

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let entry = entry.clone();
        let fname = entry.file_name().to_os_string();
        let mut file_name;
        match fname.into_string() {
            Ok(s) => {
                file_name = s.clone();
            }
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        }
        let file_name = file_name;
        let mut path_str;
        match path.to_str() {
            Some(s) => {
                path_str = format!("{}", s);
            }
            None => {
                return Err(format!("{:?}", "Path was invalid!"));
            }
        }

        if file_name.ends_with(pp_file) {
            files_to_process.push(path_str);
        } else if file_name.ends_with(pp_def) {
            src_files.push(path_str);
        }
    }

    if src_files.len() == 0 {
        return Err(format!(
            "Preprocessor can't run; no '{}' files found!",
            pp_def
        ));
    }

    Ok(())
}
