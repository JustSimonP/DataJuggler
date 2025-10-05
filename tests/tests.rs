
#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::Value;
    #[test]
    fn phrase_is_found_in_json() {
        let json_str = fs::read_to_string("tests/resources/jsons/json_deep_tree.json")
            .expect("Failed to read JSON file");

        // 2️⃣ Parse the JSON into a serde_json::Value
        let json_value: Value = serde_json::from_str(&json_str)
            .expect("Invalid JSON structure");

        // 3️⃣ Run your search logic with any test value you want
        //Import doesn't work
        let (addresses, results) = search_json_for_value(&json_value, "test_value");

        // 4️⃣ Print out results for debugging
        println!("🔍 Found paths: {:?}", addresses);
        println!("📄 Matched JSONs:\n{}", results.join("\n\n"));

        // 5️⃣ Optionally, assert that something was found
        // (Replace "expected_path" with a path that actually exists in your JSON)
        assert!(
            !addresses.is_empty(),
            "No matches found for the searched value"
        );
    }
}