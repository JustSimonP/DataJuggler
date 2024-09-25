use std::thread::Scope;
use dioxus::core_macro::component;

#[component]
fn FilterBuilder(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "bg-gray-900 p-6 w-full max-w-xl mx-auto",
            // Filter Rows
            div {
                class: "space-y-4",
                // First Filter Row
                div {
                    class: "flex items-center space-x-4",
                    // Include/Exclude Dropdown
                    select {
                        class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                        value: "{include_exclude_1}",
                        onchange: move |e| include_exclude_1.set(e.value.clone()),
                        option { value: "Include", "Include" },
                        option { value: "Exclude", "Exclude" }
                    },
                    // Property Dropdown
                    select {
                        class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                        value: "{property_1}",
                        onchange: move |e| property_1.set(e.value.clone()),
                        option { value: "Referring domain", "Referring domain" },
                        option { value: "Another property", "Another property" }
                    },
                    // Condition Dropdown
                    select {
                        class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                        value: "{condition_1}",
                        onchange: move |e| condition_1.set(e.value.clone()),
                        option { value: "Exactly matching", "Exactly matching" },
                        option { value: "Contains", "Contains" },
                        option { value: "Starts with", "Starts with" }
                    },
                    // Input Field
                    input {
                        class: "bg-gray-700 text-gray-300 py-2 px-3 rounded focus:outline-none",
                        r#type: "text",
                        placeholder: "Enter domain",
                        value: "{input_value_1}",
                        oninput: move |e| input_value_1.set(e.value.clone())
                    },
                    // Trash Icon
                    button {
                        class: "text-gray-400 hover:text-red-500",
                        "🗑️"
                    }
                },

                // Second Filter Row (repeated)
                div {
                    class: "flex items-center space-x-4",
                    select {
                        class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                        value: "{include_exclude_2}",
                        onchange: move |e| include_exclude_2.set(e.value.clone()),
                        option { value: "Include", "Include" },
                        option { value: "Exclude", "Exclude" }

    ))
}