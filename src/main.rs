use std::{env, fs};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
fn main() {
    println!("Hello, world!");
    let dir= env::current_dir().unwrap();
    let current_dir = fs::read_dir(dir).unwrap();
    let mut available_jsons = Vec::new();
    current_dir.for_each(|item| {
        let retrieved_file = item.unwrap();
        let file_path = &retrieved_file.path();
       if file_path.is_file() && file_path.extension().is_some() {
           let mir : String =  file_path.extension().and_then(OsStr::to_str).unwrap().to_string();
           println!("File format in path {}", &mir);
           if mir == "json" {
               available_jsons.push((retrieved_file.file_name().into_string().unwrap(), file_path.clone()));
           }
       }
    });
    println!("{}", available_jsons.is_empty());
   // File::open()
}

