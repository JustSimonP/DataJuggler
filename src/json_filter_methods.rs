
pub mod json_filter_methods {
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use serde_json::{Map, Value};

    fn main() -> Result<(), Box<dyn Error>> {
        let file_path = "C:/Users/User/IdeaProjects/JsonManipulator/src/generated(1).json";

        match read_json_file(file_path) {
            Ok(mappedJson) => {
                let target_value = "-138.304329";
                let mut filtered_objects = Vec::new();
                filter_objects_with_value(&mappedJson, &target_value, "",&mut filtered_objects);

                println!("Filtered Objects:");
                for obj in filtered_objects {
                    println!("{}", obj);
                }
            }
            Err(e) => eprintln!("Error reading JSON file: {}", e)
        }
        Ok(())
    }



  pub fn filter_objects_with_value(   json: &Value,
                                    target_value: &str,
                                    current_key: &str,
                                    result: &mut Vec<String>,) {
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
                    Value::Number(value) => {
                        Some(value.to_string())
                    },
                    Value::String(value) => Some(value.to_string()),
                    _ | Value::Null => None
                };
                // Check if the current value matches the target value
                if maybe_value.is_some() &&  maybe_value.unwrap() == target_value {
                    let key_value_pair = current_key.to_string() + " " + &*json.clone().to_string();
                    result.push(key_value_pair);

                }
            }
        }
    }

    pub fn retrieve_objects_by_names<'a>(
        json: &'a Value,
        matched_names: &'a Vec<String>,
    ) -> Vec<&'a Value> {
        matched_names
            .iter()
            .filter_map(|name| {
                let mut path_iter = name.split('~').peekable();
                let mut current: &Value = json;
                if name.chars().filter(|&c| c == '~').count() <= 2 {
                    return None;
                } else {
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
                            return Some(current)
                        }
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
