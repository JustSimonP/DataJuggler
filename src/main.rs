use std::{env, fs};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
// use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
fn main() {

    // let (sender, receiver) = unbounded();
    //
    // let other = sender.clone();
    dioxus_desktop::launch(App);
}
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },

        div { class: "md:container md:mx-auto border-4 border-indigo-500/100",
            "Hello, world!"
        }
    })
}

fn get_jsons() -> Vec<(String, PathBuf)>  {
    let dir = env::current_dir().unwrap();
    let current_dir = fs::read_dir(dir).unwrap();
    let mut available_jsons = Vec::new();
    current_dir.for_each(|item| {
        let retrieved_file = item.unwrap();
        let file_path = &retrieved_file.path();
        if file_path.is_file() && file_path.extension().is_some() {
            let mir: String = file_path.extension().and_then(OsStr::to_str).unwrap().to_string();
            println!("File format in path {}", &mir);
            if mir == "json" {
                available_jsons.push((retrieved_file.file_name().into_string().unwrap(), file_path.clone()));
            }
        }
    });
    available_jsons
}

