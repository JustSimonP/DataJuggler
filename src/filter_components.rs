use dioxus::core_macro::component;
use dioxus::prelude::*;

// use_ref can have collection, if we want to add new records to it we need call with_mut method
#[component]
#[allow(non_snake_case)]
pub fn SimpleFilter(cx: Scope) -> Element {
    let mut simple_filter = cx.use_hook(|| SimpleFilter {
        property: "".to_string(),
        value: "".to_string(),
    });
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

    cx.render(rsx! {
        div { class: "bg-gray-900 p-6 w-full max-w-xl mx-auto",
            div { class: "space-y-4",

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
