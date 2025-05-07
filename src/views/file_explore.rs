use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FileExploreProps {
    pick_image: EventHandler<MouseEvent>,
    selected: Signal<String>,
}

pub fn FileExplore(props: FileExploreProps) -> Element {
    // let mut selected = use_signal(|| "".to_string());

    rsx! {
        footer { class: "bg-gray-400 flex flex-col p-2 border-t text-sm",
            div { class: "flex  justify-center items-center gap-1",

                // File open button (folder icon)
                button {
                    class: "px-2 py-1 bg-white border rounded",
                    // onclick: move |_| {
                    //     if let Some(file) = FileDialog::new().pick_file() {
                    //         selected.set(file.display().to_string());
                    //     }
                    // },

                    onclick: props.pick_image,
                    "📂"
                }

                // Navigation buttons
                button { class: "text-blue-600", "⏮️" }
                button { class: "text-blue-600", "◀️" }
                button { class: "text-blue-600", "⏸️" }
                button { class: "text-blue-600", "▶️" }
                button { class: "text-blue-600", "⏭️" }

                // Green play button
                button { class: "text-green-600", "▶️" }

                // Text input and labels
                input {
                    r#type: "text",
                    class: "w-10 text-center border rounded",
                    placeholder: "",
                }
                span { "of" }

                input {
                    r#type: "text",
                    class: "w-14 text-center border rounded",
                    value: "Count",
                }

                span { "Increment" }

                input {
                    r#type: "text",
                    class: "w-6 text-center border rounded",
                    placeholder: "",
                }

                // Display selected file path (truncated)
                span { class: "ml-2 truncate text-gray-800 max-w-[200px]", "{props.selected}" }
            }
        }
    }
}
