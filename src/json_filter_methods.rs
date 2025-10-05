pub mod json_filter_methods {
    use serde_json::{Map, Value};
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use std::iter::Peekable;
    use std::str::Split;
    use dioxus::core::Scope;
    use dioxus::prelude::*;
    use crate::{FullJsonTree, ValueJsonAddresses};
    use crate::components::DisplayContents;
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let file_path = "C:/Users/User/IdeaProjects/JsonManipulator/src/generated(1).json";
    //
    //     match read_json_file(file_path) {
    //         Ok(mappedJson) => {
    //             let target_value = "-138.304329";
    //             let mut filtered_objects = Vec::new();
    //             filter_objects_with_value(&mappedJson, &target_value, "", &mut filtered_objects);
    //             let retrieve_objects = retrieve_objects_by_names(&mappedJson, &filtered_objects);
    //             println!("Filtered Objects:");
    //             for obj in filtered_objects {
    //                 println!("{}", obj);
    //             }
    //         }
    //         Err(e) => eprintln!("Error reading JSON file: {}", e),
    //     }
    //     Ok(())
    // }

    pub fn search_objects_for_value(cx: Scope, value_searched: &str) {
        println!("We are in the method, value: {}", value_searched);
        let mut json_tree: &UseSharedState<FullJsonTree> = use_shared_state::<FullJsonTree>(cx).unwrap();
        let mut json_value_addresses: &UseSharedState<ValueJsonAddresses> = use_shared_state::<ValueJsonAddresses>(cx).unwrap();
        let display_contents: &UseSharedState<DisplayContents> = use_shared_state::<DisplayContents>(cx).unwrap();

        let binding = json_tree.read();
        let (addresses, objects_in_string): (Vec<String>, Vec<String>) = search_json_for_value(&binding.deserialized_json, value_searched);

        if !addresses.is_empty() {
            json_value_addresses.write_silent().value_json_addresses = addresses;
            *display_contents.write() = DisplayContents {
                display_contents: objects_in_string.join(",\n"),
            };
        }

        }// Seems like json loading is happening and resetting the file when clicking search button
        // if result.is_empty() {
        //     let trigger_popup = move || {
        //         cx.spawn(async move {
        //             tokio::time::sleep(Duration::from_secs(4)).await;
        //         });
        //
        //         rsx!(
        //             div {
        //             class: "fixed bottom-5 right-5 bg-gray-800 text-white py-2 px-4 rounded shadow-lg opacity-100 transition-opacity duration-1000 fade-out",
        //             "Nothing found!"
        //             }
        //         )
        //     };
        // }


    pub fn search_json_for_value(json: &serde_json::Value, value_searched: &str) -> (Vec<String>, Vec<String>) {
        let mut result: Vec<String> = Vec::new();
        filter_objects_with_value(json, value_searched, "", &mut result);

        let search_results: Vec<&serde_json::Value> = retrieve_objects_by_names(json, result.clone());

        let objects_in_string: Vec<String> = search_results
            .iter()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_else(|_| "<invalid json>".to_string()))
            .collect();

        (result, objects_in_string)
    }

    pub fn filter_objects_with_value(
        json: &Value,
        target_value: &str,
        current_key: &str,
        result: &mut Vec<String>,
    ) {
        match json {
            Value::Object(map) => {
                for (key, value) in map {
                    let next_key = if current_key.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", current_key, key)
                    };
                    filter_objects_with_value(value, target_value, &next_key, result);
                }
            }
            Value::Array(vec) => {
                for (index, value) in vec.iter().enumerate() {
                    let next_key = format!("{}[{}]", current_key, index);
                    filter_objects_with_value(value, target_value, &next_key, result);
                }
            }
            other => {
                let maybe_value = match other {
                    // TODO Find more generic solution to not duplicate the code
                    Value::Bool(value) => Some(value.to_string()),
                    Value::Number(value) => Some(value.to_string()),
                    Value::String(value) => Some(value.to_string()),
                    _ | Value::Null => None,
                };
                // Check if the current value matches the target value
                if maybe_value.is_some() && maybe_value.unwrap() == target_value {
                    result.push(current_key.to_string());
                }
            }
        }
    }

    pub fn retrieve_objects_by_names<'a>(
        json: &'a Value,
        matched_names: Vec<String>,
    ) -> Vec<&'a Value> {
        matched_names
            .iter()
            .filter_map(|name| {
                println!("Name:  {}", name);
                let mut elements: Vec<&str> = name.split(".").collect();

                elements.pop();
                let mut path_iter: Peekable<Split<char>> = name.split('.').peekable();
                let mut current: &Value = json;

                    while let Some(segment) = path_iter.next() {
                        println!("OBECNY SEGMENT: {}", segment.clone().to_string());
                        if path_iter.peek().is_some() {
                            //TODO This can be way more efficient
                            if segment.ends_with("]") {
                                let index_start = segment.find('[')?;
                                let index_end = segment.find(']')?;
                                let index_str: &str = &segment[index_start + 1..index_end];
                                let index: usize = index_str.parse::<usize>().ok()?;

                                current = match current {
                                    Value::Array(vec) => {
                                        let inin = vec.get(index).expect("We are fucked!");
                                        println!("COS MAMY ARRAY: {}", inin.clone().to_string());
                                        inin
                                    }
                                    _ => return None,
                                };
                            } else {
                                current = match current {
                                    Value::Object(map) => {
                                        let dupa = map.get(segment)?;
                                        println!("COS MAMY OBJECT: {}", dupa.clone().to_string());
                                        dupa
                                    }
                                    _ => return None,
                                };
                            }
                        } else {
                            println!("COS TAM TEST !!!!: {}", current.clone().to_string());
                            return Some(current);
                        }
                    }

                Some(current)
            })
            .collect()
    }
    //look for values of object with certain name
    //look for objects of certain values
    //look for combination of fields with specific values

    fn read_json_file(file_path: &str) -> Result<Value, Box<dyn Error>> {
        // Open the file
        let file = File::open(file_path)?;

        // Create a buffered reader
        let mut reader = std::io::BufReader::new(file);

        // Read the file contents into a string
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        // Deserialize the JSON string into your data structure
        let data: Value = serde_json::from_str(&contents)?;

        Ok(data)
    }
}
