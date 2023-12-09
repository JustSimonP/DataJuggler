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

fn filter_objects_with_value(   json: &Value,
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
//look for values of object with certain name
//look for objects of certain values
//look for combination of fields with specific values