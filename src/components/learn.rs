use base64::Engine as _;
use dioxus::prelude::*;
use std::fs;
#[derive(Props, PartialEq, Clone)]
pub struct LearnSectionProps {
    image_data_url: Signal<Option<String>>,
}

fn load_svg_as_data_url(path: &str) -> String {
    let svg_bytes = fs::read(path).expect("Failed to read SVG file");
    let encoded = base64::engine::general_purpose::STANDARD.encode(svg_bytes);
    format!("data:image/svg+xml;base64,{}", encoded)
}
pub fn LearnSection(props: LearnSectionProps) -> Element {
    // Later in your component function
    let fallback_image = load_svg_as_data_url("assets/teacher.svg");

    let image_src = props
        .image_data_url
        .read()
        .clone()
        .unwrap_or(fallback_image);

    rsx! {
        section { class: "text-xs w-full flex flex-col gap-2 p-4",

            // div { class: "flex flex-col gap-2 h-32",

            //     div { class: "flex flex-wrap gap-x-10 justify-start items-start",

            //         div { class: "flex justify-start items-center gap-2",
            //             button { class: "bg-white p-3 rounded border shadow", "Learn" }
            //         }

            //         div { class: "flex flex-col gap-2",
            //             label { "Learn Mode" }
            //             select { class: "border rounded",
            //                 option { "Codebook" }
            //                 option { "Annotations" }
            //             }
            //         }

            //         div { class: "flex flex-col gap-2",
            //             label { "Categories" }
            //             select { class: "border rounded",
            //                 option { "Increment" }
            //                 option { "Constant" }
            //                 option { "MaxDeltaAmplitude" }
            //                 option { "AvgAmplitude" }
            //                 option { "InputIndex" }
            //             }
            //         }
            //     }

            //     div { class: "flex flex-wrap gap-10 justify-start items-center",

            //         div { class: "flex justify-start items-center gap-2",
            //             input { r#type: "checkbox", id: "auto" }
            //             label { r#for: "auto", "Auto" }
            //         }

            //         div { class: "flex justify-start items-center gap-2",
            //             button { class: "co bg-white px-3 py-1 rounded border shadow",
            //                 "Select full image"
            //             }
            //         }

            //         div { class: "flex justify-start items-  gap-2 -ml-5",
            //             label { "StepXY" }
            //             input {
            //                 r#type: "number",
            //                 value: "16",
            //                 class: "w-12 p-1 border rounded",
            //             }
            //             input {
            //                 r#type: "number",
            //                 value: "16",
            //                 class: "w-12 p-1 border rounded",
            //             }
            //         }
            //     }
            // }


            div { class: "w-3/4 grid grid-rows-2 gap-4 h-32 items-start justify-items-start",

                // First Row
                div { class: "w-full grid grid-cols-3 gap-x-10 items-start",

                    // Learn Button
                    div { class: "flex items-center gap-2",
                        button { class: "bg-white p-4 rounded border shadow", "Learn" }
                    }

                    // Learn Mode
                    div { class: "flex flex-col gap-2",
                        label { "Learn Mode" }
                        select { class: "border rounded",
                            option { "Codebook" }
                            option { "Annotations" }
                        }
                    }

                    // Categories
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

                // Second Row
                div { class: "w-full grid grid-cols-3 gap-x-10 items-center justify-start",

                    // Auto Checkbox
                    div { class: "flex items-center gap-2",
                        input { r#type: "checkbox", id: "auto" }
                        label { r#for: "auto", "Auto" }
                    }

                    // Select Full Image Button
                    div { class: "flex items-center gap-2",
                        button { class: "bg-white text-xs px-2 py-1 rounded border shadow",
                            "Select full image"
                        }
                    }

                    // StepXY Inputs
                    div { class: "flex items-center gap-2",
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
            div { class: "bg-white flex-1 h-[400px] flex items-center justify-center",
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
