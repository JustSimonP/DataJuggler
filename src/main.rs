mod json_filter_methods;

use std::{env, fs};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, Read};
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_desktop::tao::dpi::{PhysicalPosition, Position};
use serde_json::ser::State;
use serde_json::Value;
use winit::dpi::PhysicalSize;
use winit::monitor::MonitorHandle;


use crate::json_filter_methods::json_filter_methods::filter_objects_with_value;

// use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
fn main() {


    // withPosition method describes position for top left cover part of displayed window
    let (window_width, window_height, center_position) = getWindowSizeWithPosition();
    dioxus_desktop::launch_cfg(app,
                               Config::default()
                                   .with_window(WindowBuilder::new()
                                   .with_resizable(true)
                                   .with_inner_size(
                                   dioxus_desktop::wry::application::dpi::PhysicalSize::new(window_width, window_height),
                               )
                                   .with_title("Json Juggler")
                                   .with_position(center_position)
                               ));

    // dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {

    // let  json_name_to_path_map: HashMap<String, PathBuf> = get_jsons();
    let json_name_to_path_map: &mut HashMap<String, PathBuf> = cx.use_hook(|| get_jsons());
    use_shared_state_provider(cx, || JsonViewState::FileNotChosen);
    let json_view_state: &UseSharedState<JsonViewState> = use_shared_state::<JsonViewState>(cx).unwrap();

    // use_shared_state_provider(cx, || JsonPath {maybe_json_path: None});

    cx.render(rsx! {
                 // link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
                // style { include_str!("./style.css") }
                div {

                width:"100%",
                display:"flex",

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

fn getWindowSizeWithPosition() -> (u32, u32, Position) {
    let event_loop = winit::event_loop::EventLoop::new();

    // Get the primary monitor
    let primary_monitor: MonitorHandle = event_loop.primary_monitor().unwrap();
    let monitor_size: PhysicalSize<u32> = primary_monitor.size();

    let center_x = (monitor_size.width /2) as i32;
    let center_y = (monitor_size.height /2) as i32;

    // Calculate the desired width and height (80% of the screen size)
    let window_width = (monitor_size.width as f64 * 0.8) as u32;
    let window_height = (monitor_size.height as f64 * 0.8) as u32;

    let corner_position_x = center_x - (window_width as i32/ 2);
    let corner_position_y = center_y - (window_height as i32/ 2);

    let center_position: Position =  Position::Physical(PhysicalPosition::new(corner_position_x as i32,corner_position_y as i32));

    (window_width, window_height, center_position)
}

fn get_jsons() -> HashMap<String, PathBuf> {
    let dir = env::current_dir().unwrap();
    let current_dir = fs::read_dir(dir).unwrap();
    let mut available_jsons = HashMap::new();
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
    pub(crate) maybe_json_path: PathBuf,
}

#[component]
fn JsonView(cx: Scope) -> Element {
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();
    match &*json_view_state.read() {
        JsonViewState::Loaded(path) => {
            let file = File::open(path.maybe_json_path.clone().as_path()).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents: std::string::String = String::new();
            // let serde_json_string = serde_json::from_reader(&buf_reader).unwrap();
            buf_reader.read_to_string(&mut contents).expect("Unable to read the file");
            let is_json_formatted = &contents[0..6].find("\n").is_some();

            use_shared_state_provider(cx,  || serde_json::from_str(contents));
            //from_reader can be used to deserialize directly from the file
            // let formatted_json: &mut State = cx.use_hook(||serde_json::from_str(&format!("\"{}\"", &contents)).unwrap());
            // println!("json: {}",&formatted_json.);
            render! {
            SearchBox{}
            div {
                    white_space: "pre-wrap",
                    padding: "20px",
                    background_color: "lightgray",
              "{contents}"
            }
        }
        }
        _ => render! {
            div {
                "dupa"
            }
        }
    }
}
#[component]
pub fn SearchBox(cx: Scope) -> Element {
    let mut searchValue = cx.use_hook(|| "");
    let getData = move || {
        println!("mfniecmiwmi");
    };
    render! {
        div {
                display: "flex",
                flex_direction: "row",

            input {
                placeholder: "Type the value you want to search in document",
                value: "{searchValue}",
                border: "3px",
                position: "relative"
            }
            button {
                content: "+",
                background_color: "lightblue",

            }
            button {
                content: "find",
                onclick: move |event| {
                    find_json_by_phrase(&cx);
                }

            }
        }
    }
}

 fn find_json_by_phrase(cx: Scope<'_>) {

    let json_view_state: &UseSharedState<JsonViewState> = use_shared_state::<JsonViewState>(cx).unwrap();
     filter_objects_with_value()
    *json_view_state.write() = JsonViewState::FileNotChosen
}

//check if recursive call is optimized in rust, maybe benchmark
// check if flatten from serde will be useful here
fn deserialize_and_find(json: Value) -> Option<String> {
    use serde_json::Value::{Array, Bool, Number, Object, String, Null};
    match json {
        Bool(some_boolean) => None,
        Number(some_int) => None,
        String(some_string) => None,
        Array(json_array) => None,
        Object(json_map) => None,
        Null => None,
    }
}