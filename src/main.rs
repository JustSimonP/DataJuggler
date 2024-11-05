mod filter_components;
mod json_filter_methods;

use dioxus::prelude::*;
use dioxus_desktop::tao::dpi::{PhysicalPosition, Position};
use dioxus_desktop::{Config, WindowBuilder};
use serde_json::ser::State;
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fs};
use winit::dpi::PhysicalSize;
use winit::monitor::MonitorHandle;

use crate::filter_components::SimpleFilter;

use crate::json_filter_methods::json_filter_methods::filter_objects_with_value;

// use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
fn main() {
    // withPosition method describes position for top left cover part of displayed window
    let (window_width, window_height, center_position) = getWindowSizeWithPosition();
    dioxus_desktop::launch_cfg(
        app,
        Config::default()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
            .with_window(
                WindowBuilder::new()
                    .with_resizable(true)
                    .with_inner_size(dioxus_desktop::wry::application::dpi::PhysicalSize::new(
                        window_width,
                        window_height,
                    ))
                    .with_title("Json Juggler")
                    .with_position(center_position),
            ),
    );

    // dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let json_name_to_path_map: &mut HashMap<String, PathBuf> = cx.use_hook(|| get_jsons());
    use_shared_state_provider(cx, || JsonViewState::FileNotChosen);
    use_shared_state_provider(cx, || DisplayContents {
        display_contents: String::new(),
    });
    use_shared_state_provider(cx, || FullJsonTree { deserialized_json: Value::Null });
    let json_view_state: &UseSharedState<JsonViewState> =
        use_shared_state::<JsonViewState>(cx).unwrap();
    cx.render(rsx! {

                div {
                    class: "flex flex-row w-full h-full",

                    div {
                        class: "w-1/5 overflow-y-auto p-2",
                        border_style: "solid",
                        border_width: "1px",
                        border_color: "black",

                        json_name_to_path_map.iter().map(|(file_name, file_path)| rsx! {
                            ul {
                                li{ onclick: move |event| {
                                    *json_view_state.write() = JsonViewState::Loaded(JsonPath{maybe_json_path: file_path.clone()})}
                                ,
                                "{file_name}"
                                    }
                            }})
                    }
                    div {
                        div {
                            SearchBox{}
                        }
                        div {
                            JsonView {}
                        }
                    }

                }
    })
}

fn getWindowSizeWithPosition() -> (u32, u32, Position) {
    let event_loop = winit::event_loop::EventLoop::new();

    // Get the primary monitor
    let primary_monitor: MonitorHandle = event_loop.primary_monitor().unwrap();
    let monitor_size: PhysicalSize<u32> = primary_monitor.size();

    let center_x = (monitor_size.width / 2) as i32;
    let center_y = (monitor_size.height / 2) as i32;

    // Calculate the desired width and height (80% of the screen size)
    let window_width = (monitor_size.width as f64 * 0.8) as u32;
    let window_height = (monitor_size.height as f64 * 0.8) as u32;

    let corner_position_x = center_x - (window_width as i32 / 2);
    let corner_position_y = center_y - (window_height as i32 / 2);

    let center_position: Position = Position::Physical(PhysicalPosition::new(
        corner_position_x as i32,
        corner_position_y as i32,
    ));

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
            let mir: String = file_path
                .extension()
                .and_then(OsStr::to_str)
                .unwrap()
                .to_string();
            println!("File format in path {}", &mir);
            if mir == "json" {
                available_jsons.insert(
                    retrieved_file.file_name().into_string().unwrap(),
                    file_path.clone(),
                );
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

#[derive(Clone, Debug)]
struct FullJsonTree {
    pub deserialized_json: Value,
}

#[derive(Clone, Debug)]
struct DisplayContents {
    pub display_contents: String,
}

#[derive(Clone, Debug)]
struct RowsToDisplay {
    pub display_text: String,
}

#[component]
fn JsonView(cx: Scope) -> Element {
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();
    let display_contents = use_shared_state::<DisplayContents>(cx).unwrap();
    let deserialized_structure = use_shared_state::<FullJsonTree>(cx).unwrap();
    match &*json_view_state.read() {
        JsonViewState::Loaded(path) => {
            let file = File::open(path.maybe_json_path.clone().as_path()).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader
                .read_to_string(&mut contents)
                .expect("Unable to read the file");
            let is_json_formatted = &contents[0..6].find("\n").is_some();

            // Czy uzycie use_ref nie będzie lżejsze
            if *is_json_formatted {
                println!("JAPIERDOLE");
                deserialized_structure.write_silent().deserialized_json = serde_json::from_str::<Value>(contents.as_str()).unwrap();
            } else {
                println!("CHUJSA");
                contents = jsonxf::pretty_print(contents.as_str()).unwrap();
                deserialized_structure.write_silent().deserialized_json = serde_json::from_str::<Value>(contents.as_str()).unwrap();
            }
            display_contents.write_silent().display_contents = contents;

            render! {
                div {
                    border_style: "solid",
                    border_width: "1px",
                    border_color: "black",
                    // class: "flex flex-col w-3/4 p-4",
                    // div {
                    //     class:"flex items-center space-x-2",
                    //     SearchBox{}
                    // }

                    // SimpleFilter{}
                    div {
                        // class: "w-full mt-4 overflow-y-auto p-4 bg-gray-100 rounded",
                        white_space: "pre-wrap",
                        // padding: "20px",
                        background_color: "lightgray",
                        "{display_contents.read().display_contents}"
                    }
                }
            }
        }
        _ => render! {
            div {
                class: "flex flex-col w-3/4 p-4",
                "dupa"
            }
        },
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
            class: "flex flex-row",
            div {
                class: "p-2 border rounded resize-y",
                input {

                    placeholder: "Type the value you want to search in document",
                    value: "d",
                    // border: "3px",
                    // position: "relative"
                }
            }
           div {
                class: "absolute right-2 top-1/2 transform -translate-y-1/2 bg-blue-500 text-white p-2 rounded",
                button {
                content: "search",
                onclick: move |event| {
                    // find_json_by_phrase(&cx);
                    }
                }
            }

        }
    }
}

//  fn find_json_by_phrase(cx: Scope<'_>) {
//
//     let json_view_state: &UseSharedState<JsonViewState> = use_shared_state::<JsonViewState>(cx).unwrap();
//      filter_objects_with_value()
//     *json_view_state.write() = JsonViewState::FileNotChosen
// }

//check if recursive call is optimized in rust, maybe benchmark
// check if flatten from serde will be useful here
fn deserialize_and_find(json: Value) -> Option<String> {
    use serde_json::Value::{Array, Bool, Null, Number, Object, String};
    match json {
        Bool(some_boolean) => None,
        Number(some_int) => None,
        String(some_string) => None,
        Array(json_array) => None,
        Object(json_map) => None,
        Null => None,
    }
}
