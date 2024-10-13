use dioxus::core_macro::component;
use dioxus::prelude::*;


// use_ref can have collection, if we want to add new records to it we need call with_mut method
#[component]
#[allow(non_snake_case)]
pub fn SimpleFilter(cx: Scope) -> Element {
    let mut simple_filter = cx.use_hook(|| SimpleFilter { property: "".to_string(), value: "".to_string() });
    render! {
         div { class: "bg-gray-900 p-6 w-full max-w-xl mx-auto",
            div {
                class: "flex items-center space-x-4",
                select {
                            class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                            value: "{simple_filter.property",
                            oninput: move |e| println!("DUPA"),
                            option { value: "Include", "Include" }
                            option { value: "Exclude", "Exclude" }
                        }
            }
        }

    }
}

#[component]
#[allow(non_snake_case)]
pub fn FilterBuilder(cx: Scope) -> Element {
    let filters = use_ref(cx, || vec![Filter::default()]);
    let add_filter = {
        let filters = filters.clone();
        move |_| filters.write().push(Filter::default())
    };

    let update_filter = {
        let filters = filters.clone();
        move |(index, field, new_value): (usize, &str, String)| {
            if let Some(filter) = filters.read().get(index) {
                match field {
                    "include" => filter.include = new_value,
                    "property" => filter.property = new_value,
                    "condition" => filter.condition = new_value,
                    "value" => filter.value = new_value,
                    _ => {}
                }
            }
        }
    };

    fn remove_filter(filters: &UseRef<Vec<Filter>>, index: usize) {
        filters.write().remove(index);
    }


    let save_filters = {
        let filters = filters.clone();
        move |_| {
            // Replace this with actual saving logic
            println!("DUPA");
        }
    };

    cx.render(rsx! {
        div { class: "bg-gray-900 p-6 w-full max-w-xl mx-auto",
            div { class: "space-y-4",
                filters.read().iter().enumerate().map(|(index, filter)| rsx!(
                    div { class: "flex items-center space-x-4",
                        // Include/Exclude Dropdown
                        select {
                            class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                            value: "{filter.include}",
                            oninput: &move |e| update_filter((index, "include", e.value.clone())),
                            option { value: "Include", "Include" }
                            option { value: "Exclude", "Exclude" }
                        }

                        // Property Dropdown
                        select {
                            class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                            value: "{filter.property}",
                            oninput: &move |e| update_filter((index, "property", e.value.clone())),
                            option { value: "Referring domain", "Referring domain" }
                            option { value: "Another property", "Another property" }
                        }

                        // Condition Dropdown
                        select {
                            class: "bg-gray-700 text-gray-300 py-2 px-3 rounded",
                            value: "{filter.condition}",
                            oninput: &move |e| update_filter((index, "condition", e.value.clone())),
                            option { value: "Exactly matching", "Exactly matching" }
                            option { value: "Contains", "Contains" }
                            option { value: "Starts with", "Starts with" }
                        }

                        // Input Field
                        input {
                            class: "bg-gray-700 text-gray-300 py-2 px-3 rounded focus:outline-none",
                            value: "{filter.value}",
                            placeholder: "Enter domain",
                            oninput: &move |e| update_filter((index, "value", e.value.clone())),
                        }

                        // Trash Icon (Delete Filter)
                        button {
                            class: "text-gray-400 hover:text-red-500",
                            onclick: &move |_| remove_filter(filters, index),
                            "🗑️"
                        }
                    }
                ))
            }

            // Add Property Button
            div { class: "mt-4",
                button {
                    class: "text-blue-500 hover:underline",
                    onclick: add_filter,
                    "+ Add Property"
                }
            }

            // Apply & Clear Buttons
            div { class: "flex justify-between items-center mt-6",
                button {
                    class: "bg-blue-500 text-white py-2 px-6 rounded hover:bg-blue-600",
                    onclick: save_filters,
                    "Apply"
                }
                button {
                    class: "text-gray-400 hover:text-red-500",
                    onclick: move |_| filters.set(vec![Filter::default()]),
                    "Clear all"
                }
            }
        }
    })
}

#[derive(Clone, Debug)]
pub struct Filter {
    include: String,
    property: String,
    condition: String,
    value: String,
}

#[derive(Clone, Debug)]
pub struct SimpleFilter {
    property: String,
    value: String,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            include: "Include".to_string(),
            property: "Referring domain".to_string(),
            condition: "Exactly matching".to_string(),
            value: "".to_string(),
        }
    }
}