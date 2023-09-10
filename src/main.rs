
use std::{env, fs};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Map;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use serde_json::ser::Compound::Map;
use serde_json::Value;


// use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
fn main() {

    // let (sender, receiver) = unbounded();
    //
    // let other = sender.clone();
    dioxus_desktop::launch(app);
}
fn app(cx: Scope) -> Element {

    // let  json_name_to_path_map: HashMap<String, PathBuf> = get_jsons();
    let  json_name_to_path_map: &mut HashMap<String, PathBuf> = cx.use_hook(||get_jsons());
    use_shared_state_provider(cx, || JsonViewState::FileNotChosen);
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();

    // use_shared_state_provider(cx, || JsonPath {maybe_json_path: None});

    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            align_items: "stretch",

            div {
                width: "20%",
                border_style: "solid",
                border_width: "1px",
                border_color: "black",

            json_name_to_path_map.iter().map(|(file_name, file_path)| rsx! {
                ul {
                    li{onclick: move |event| {
                      // let kil: Option<PathBuf> =  match json_name_to_path_map.get(file_name) {
                      //       Some(dupa) => Some(dupa.clone()),
                      //       _ => None
                      //   };
                        *json_view_state.write() = JsonViewState::Loaded(JsonPath{maybe_json_path: file_path.clone()})}
                    ,
                    "{file_name}"
                        }
                }})
            }
            div {
                border_style: "solid",
                border_width: "1px",
                border_color: "black",
                width: "80%",
                JsonView {}
            }
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
    pub(crate) maybe_json_path: PathBuf
}

#[inline_props]
fn JsonView(cx : Scope) -> Element  {
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();
    match &*json_view_state.read() {
        JsonViewState::Loaded(path) => {
            let file = File::open(path.maybe_json_path.clone().as_path()).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents: std::string::String = String::new();
            let serde_json_string =  serde_json::from_reader(&buf_reader).unwrap();
            buf_reader.read_to_string(&mut contents).expect("Unable to read the file");
            let is_json_formatted = &contents[0..6].find("\n").is_some();
            //from_reader can be used to deserialize directly from the file
            //let formatted_json = cx.use_hook(||serde_json::from_str(&format!("\"{}\"", &contents)).unwrap());
            //println!("json: {}",&formatted_json);
            render! {

            div {
                    white_space: "pre-wrap",
              "{contents}"
            }
        }
        },
        _ => render! {
            div {
                "dupa"
            }
        }
    }
}

//check if recursive call is optimized in rust, maybe benchmark
// check if flatten from serde will be useful here
fn deserialize_and_find(json: Value) -> Option<String> {
    use serde_json::Value::{Array, Bool, Number, Object, String, Null};
     match json {

             Bool(some_boolean)=> None,
             Number(some_int) => None,
             String(some_string) => None,
             Array(json_array) => None,
             Object(json_map) => None,
             Null => None,

     }
}