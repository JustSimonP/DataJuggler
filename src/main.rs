use std::{env, fs};
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
   // let dir= env::current_dir().unwrap();
    let current_dir = fs::read_dir("C:/Users/User").unwrap();
    current_dir.for_each(|item| {
        let mk = item.unwrap().path();
       if mk.is_file() && mk.extension().is_some() {
           let mir : String =  mk.extension().and_then(OsStr::to_str).unwrap().to_string();
           println!("ifeiifi {}", &mir)
       }
    })
    // let huj = fs::read_dir(current_dir).unwrap();
}
// fn listJsonsByDateModified() {
//     let current_dir = env::current_dir().unwrap().as_ref();
//
//        // entry.unwrap().metadata().unwrap().file_type()
//     let huj = fs::read_dir(current_dir).unwrap();
//     // let jsons = fs::read_dir(current_dir)
//     //     .unwrap().filter(|x| x.unwrap().metadata().unwrap().file_type() == "json" );
// }


// println!("hello there!");
// println!("format {} arguments", "some");
// let local_variable = "some";
// println!("format {local_variable} arguments");