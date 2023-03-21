use std::{env, fs};
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    let dir= env::current_dir().unwrap();
    let current_dir = fs::read_dir(dir).unwrap();
    current_dir.for_each(|item| {
        let mk = item.unwrap().path();
       if mk.is_file() && mk.extension().is_some() {
           let mir : String =  mk.extension().and_then(OsStr::to_str).unwrap().to_string();
           println!("File format in path {}", &mir)
       }
    })
}

