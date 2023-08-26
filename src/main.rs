use std::{env, fs};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::iter::Map;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
// use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
fn main() {

    // let (sender, receiver) = unbounded();
    //
    // let other = sender.clone();
    dioxus_desktop::launch(app);
}
fn app(cx: Scope) -> Element {

    let json_name_to_path_map: HashMap<String, PathBuf> = get_jsons();
    use_shared_state_provider(cx, || JsonViewState::FileNotChosen);
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();

    // use_shared_state_provider(cx, || JsonPath {maybe_json_path: None});

    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },

        div {

            json_name_to_path_map.keys().into_iter().map(|file_name| rsx! {
                ul {
                    li{onclick: move |event| {
                      let kil: Option<PathBuf> =  match json_name_to_path_map.get(file_name) {
                            Some(dupa) => Some(dupa.clone()),
                            _ => None
                        };
                        *json_view_state.write() = JsonViewState::Loaded(JsonPath{maybe_json_path: kil})}
                    ,
                    "{file_name}"}
                }})
        }
         // json_name_to_path_map.keys.map(|x.|)
    })
}
// async fn determinePath(dupa: UseRef<HashMap<String, PathBuf>>) {
//
// }
fn get_jsons() -> HashMap<String, PathBuf>  {
    let dir = env::current_dir().unwrap();
    let current_dir = fs::read_dir(dir).unwrap();
    let mut available_jsons= HashMap::new();
    current_dir.for_each(|item| {
        let retrieved_file = item.unwrap();
        let file_path = &retrieved_file.path();
        if file_path.is_file() && file_path.extension().is_some() {
            let mir: String = file_path.extension().and_then(OsStr::to_str).unwrap().to_string();
            println!("File format in path {}", &mir);
            if mir == "json" {
                available_jsons.insert(retrieved_file.file_name().into_string().unwrap(), file_path.clone());
            }
        }
    });
    available_jsons
}

#[derive(Clone, Debug)]
enum JsonViewState {
    FileNotChosen,
    Loading,
    Loaded(JsonPath),
}

#[derive(Clone, Debug)]
struct JsonPath {
    maybe_json_path: Option<PathBuf>
}

#[inline_props]
fn JsonView(cx : Scope) -> Element  {
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();
    match &*json_view_state.read() {
        JsonViewState::Loaded(path) => println!("Its working"),
        _ => println!("DUPA")
    }
}
