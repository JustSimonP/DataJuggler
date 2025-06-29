mod filter_components;

mod json_filter_methods;
mod components;

use json_filter_methods::json_filter_methods::search_objects_for_value;
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

use crate::components::DisplayContents;

// use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
fn main() {
    // withPosition method describes position for top left cover part of displayed window
    let (window_width, window_height, center_position) = get_window_size_with_position();
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

}

fn app(cx: Scope) -> Element {
    println!("Rendering main component!");
    use_shared_state_provider(cx, || JsonViewState::FileNotChosen);
    use_shared_state_provider(cx, || DisplayContents {
        display_contents: String::new(),
    });
    use_shared_state_provider(cx, || FullJsonTree { deserialized_json: Value::Null });
    use_shared_state_provider(cx, || ValueJsonAddresses { value_json_addresses: Vec::new()});
    // let json_view_state: &UseSharedState<JsonViewState> =
    //     use_shared_state::<JsonViewState>(cx).unwrap();
    let json_name_to_path_map: &HashMap<String, PathBuf> = use_memo(cx, (), |_| get_jsons());

    cx.render(rsx! {

                div {
                    class: "flex flex-row w-screen h-screen min-h-screen min-w-screen",

                    Sidebar{json_name_to_path_map: json_name_to_path_map.clone()}
                    div {
                        class: "w-4/5 h-full flex flex-col p-4 space-y-4",
                        SearchBox{},
                        JsonView{}
                    }

                }
    })
}

#[derive(Props, PartialEq)]
pub struct SidebarProps {
    json_name_to_path_map: HashMap<String, PathBuf>,
}

fn Sidebar(cx: Scope<SidebarProps>) -> Element {
    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();
    println!("Rendering Sidebar component!");
    cx.render(rsx! {
        div {
            class: "w-1/5 h-full p-2 border border-black flex flex-col",

            div {
                class: "flex-grow overflow-y-auto overflow-x-auto border-b border-black",

                cx.props.json_name_to_path_map.iter().map(|(file_name, file_path)| rsx! {
                    ul {
                        li {
                            onclick: move |_| {
                                *json_view_state.write() =
                                    JsonViewState::Loaded(JsonPath {
                                        maybe_json_path: file_path.clone()
                                    });
                            },
                            "{file_name}"
                        }
                    }
                })
            },
            div {
                class: "flex-grow overflow-y-auto",
                ul {
                    li { "test" },
                    li { "test" },
                }
            }
        }
    })
}

fn get_window_size_with_position() -> (u32, u32, Position) {
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

#[derive(Clone, Debug, PartialEq)]
enum JsonViewState {
    FileNotChosen,
    Loading,
    Loaded(JsonPath),
}

#[derive(Clone, Debug, PartialEq)]
struct JsonPath {
    pub(crate) maybe_json_path: PathBuf,
}

#[derive(Clone, Debug)]
struct FullJsonTree {
    pub deserialized_json: Value,
}



#[derive(Clone, Debug)]
struct RowsToDisplay {
    pub display_text: String,
}

struct ValueJsonAddresses {
    pub value_json_addresses: Vec<String>,
}
// #[component]
// fn JsonValueAddresses(cx: Scope) -> Element {
//     let addresses = use_shared_state::<ValueJsonAddresses>(cx).unwrap();
//
//     addresses.read().value_json_addresses.map(|address| rsx!(
//
//     ))
// }

#[component]
fn FileDisplay(cx: Scope) -> Element {
    println!("FileDisplay component rendered");

    let display_contents = use_shared_state::<DisplayContents>(cx).unwrap();

    cx.render(rsx! {
                    div {
                        class:"w-full p-4 border border-gray-300 bg-gray-100 overflow-y-auto resize-y",
                        // class: "w-full mt-4 overflow-y-auto p-4 bg-gray-100 rounded",
                        white_space: "pre-wrap",
                        // padding: "20px",
                        background_color: "lightgray",
                        "{display_contents.read().display_contents}"
                    }

            })
}

//Now clicking at diffferent json path doesn't rerender display
#[component]
fn JsonView(cx: Scope) -> Element {
    println!("Rendering JsonView");

    let json_view_state = use_shared_state::<JsonViewState>(cx).unwrap();

    // This memo will recompute when the JsonViewState changes
    let _ = use_memo(cx, &[json_view_state.read().clone()], move |_| {
        if let JsonViewState::Loaded(path) = &*json_view_state.read() {
            println!("Loading and parsing JSON");

            let file = File::open(path.maybe_json_path.as_path()).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();

            let is_json_formatted = contents.find('\n').is_some();
            let pretty = if !is_json_formatted {
                jsonxf::pretty_print(&contents).unwrap()
            } else {
                contents.clone()
            };

            // Write data to shared states
            let deserialized_structure = use_shared_state::<FullJsonTree>(cx).unwrap();
            let display_contents = use_shared_state::<DisplayContents>(cx).unwrap();

            deserialized_structure.write().deserialized_json =
                serde_json::from_str(&pretty).unwrap();
            display_contents.write().display_contents = pretty;
        }
    });

    match &*json_view_state.read() {
        JsonViewState::Loaded(_) => {
            cx.render(rsx! {
                FileDisplay {}
            })
        }
        _ => cx.render(rsx! {
            div { "No file selected" }
        }),
    }
}
// Putting use_shared_state in separate method causes the component not to re-render
fn update_display_contents( contents: &str, cx: Scope) {
    let deserialized_structure = use_shared_state::<FullJsonTree>(cx).unwrap();
    let display_contents = use_shared_state::<DisplayContents>(cx).unwrap();

    let parsed_json: Value = serde_json::from_str(contents).unwrap();
    deserialized_structure.write_silent().deserialized_json = parsed_json;
    display_contents.write_silent().display_contents = contents.to_string();
}

#[component]
pub fn SearchBox(cx: Scope) -> Element {
    let mut search_value = use_state(cx, || "".to_string());
    println!("{}", *search_value);

    render! {
                div {
                         class: "relative flex items-center w-full space-x-2",
                            textarea {
                                class:"flex-grow p-2 border border-gray-300 rounded resize-y min-h-fit",
                                rows:"1",
                                placeholder: "Describe needed value/property here",
                                oninput: move |event| search_value.set(event.value.clone())
                            }
                            button {
                            class:"bg-blue-500 text-white p-2 rounded",
                            style:"top 50%",
                            onclick: move |_| {
                                    search_objects_for_value(cx, &search_value.get())
                                 },
                            "Search"

                         }
                        }
    }
}

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
