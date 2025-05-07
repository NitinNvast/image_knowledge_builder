use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let items = [
        "Project",
        "Annotations",
        "Knowledge",
        "Results",
        "Options",
        "Help",
    ];

    // Map the items into a Vec of rsx elements
    let buttons = items.iter().map(|item| {
        rsx!(
            button {
                class: "hover:bg-blue-100 hover:border",
                "{item}"
            }
        )
    });

    rsx! {
        nav {
            class: "flex gap-4 p-1 bg-gray-50 text-sm",
            // Spread the buttons into the rsx
            {buttons}
        }
    }
}
