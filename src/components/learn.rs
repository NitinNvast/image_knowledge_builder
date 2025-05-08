use dioxus::{html::div, prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct LearnSectionProps {
    image_data_url: Signal<Option<String>>,
}

pub fn LearnSection(props: LearnSectionProps) -> Element {
    let image_src = props.image_data_url.read().clone().unwrap_or(
        "/home/nitin/Nitin_Personal/Rust/image_knowledge_builder/assets/teacher.svg".to_string(),
    );

    rsx! {
        section { class: "w-full flex flex-col gap-2 p-4 text-sm",

            div { class: "flex flex-col gap-2 h-32",

                div { class: "flex flex-wrap gap-x-10 justify-start items-start",

                    div { class: "flex justify-start items-center gap-2",
                        button { class: "bg-white p-3 rounded border shadow", "Learn" }
                    }

                    div { class: "flex flex-col gap-2",
                        label { "Learn Mode" }
                        select { class: "border rounded",
                            option { "Codebook" }
                            option { "Annotations" }
                        }
                    }

                    div { class: "flex flex-col gap-2",
                        label { "Categories" }
                        select { class: "border rounded",
                            option { "Increment" }
                            option { "Constant" }
                            option { "MaxDeltaAmplitude" }
                            option { "AvgAmplitude" }
                            option { "InputIndex" }
                        }
                    }
                }

                div { class: "flex flex-wrap gap-10 justify-start items-center",

                    div { class: "flex justify-start items-center gap-2",
                        input { r#type: "checkbox", id: "auto" }
                        label { r#for: "auto", "Auto" }
                    }

                    div { class: "flex justify-start items-center gap-2",
                        button { class: "co bg-white px-3 py-1 rounded border shadow",
                            "Select full image"
                        }
                    }

                    div { class: "flex justify-start items-  gap-2 -ml-5",
                        label { "StepXY" }
                        input {
                            r#type: "number",
                            value: "16",
                            class: "w-12 p-1 border rounded",
                        }
                        input {
                            r#type: "number",
                            value: "16",
                            class: "w-12 p-1 border rounded",
                        }
                    }
                }
            }


            // Image preview placeholder
            div { class: "bg-white flex-1 h-[400px] mt-4 flex items-center justify-center",
                img {
                    // src: "/path/to/learn.png", // Replace with actual image URL or logic
                    src: image_src,
                    alt: "Learn Image",
                    class: "object-contain max-h-full",
                }
            }
        }
    }
}
