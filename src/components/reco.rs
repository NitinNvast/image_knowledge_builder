use base64::Engine as _;
use dioxus::prelude::*;
use std::fs;
#[derive(Props, PartialEq, Clone)]
pub struct RecoSectionProps {
    image_data_url: Signal<Option<String>>,
}

fn load_svg_as_data_url(path: &str) -> String {
    let svg_bytes = fs::read(path).expect("Failed to read SVG file");
    let encoded = base64::engine::general_purpose::STANDARD.encode(svg_bytes);
    format!("data:image/svg+xml;base64,{}", encoded)
}

pub fn RecoSection(props: RecoSectionProps) -> Element {
    // Later in your component function
    let fallback_image = load_svg_as_data_url("assets/lens.svg");

    let image_src = props
        .image_data_url
        .read()
        .clone()
        .unwrap_or(fallback_image);

    rsx! {
        section { class: "w-full text-xs flex flex-col gap-6 p-4 text-sm",

            // Top bar: Learn button, dropdowns, select image, auto checkbox
            div { class: "flex flex-wrap  justify-start gap-10 h-28",

                // div { class: "flex flex-col gap-2",

                //     div { class: "flex flex-wrap gap-10 justify-start items-center",

                //         div { class: "flex justify-start items-center gap-2",
                //             button { class: "bg-white p-3 rounded border shadow", "Reco" }
                //         }

                //         div { class: "flex flex-col gap-2",
                //             label { "Output Mode" }
                //             select { class: "border rounded",
                //                 option { "Objects" }
                //                 option { "Clusters" }
                //                 option { "Anomalies" }
                //                 option { "MapOfCategories" }
                //                 option { "MapOfModels" }
                //                 option { "MapOfDistances" }
                //                 option { "MapOfIdentifiers" }
                //             }
                //         }

                //     }

                //     div { class: "flex flex-wrap gap-10 justify-start items-center",

                //         div { class: "flex justify-start items-center gap-2",
                //             input { r#type: "checkbox", id: "auto" }
                //             label { r#for: "auto", "Auto" }
                //         }

                //         div { class: "flex justify-start items-center gap-2",
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

                //             span { class: "flex items-center gap-1",
                //                 input { r#type: "checkbox", id: "skipx" }
                //                 label { r#for: "skipx", "SkipX" }
                //             }

                //             span { class: "flex items-center gap-1",
                //                 input { r#type: "checkbox", id: "skipy" }
                //                 label { r#for: "skipy", "SkipY" }
                //             }
                //         }
                //     }

                //     div { class: "flex flex-wrap gap-10 justify-start items-center",

                //         div { class: "flex justify-start items-center gap-2",
                //             input { r#type: "checkbox", id: "fullimage" }
                //             label { r#for: "fullimage", "Full image" }
                //         }
                //     }
                // }


                div { class: "w-3/4 grid grid-rows-2 gap-4 h-32 items-start justify-items-start",

                    // First Row
                    div { class: "w-full grid grid-cols-3 gap-x-10 items-start",

                        // Learn Button
                        div { class: "flex items-center gap-2",
                            button { class: "bg-white p-4 rounded border shadow", "Reco" }
                        }

                        // Categories
                        div { class: "flex flex-col gap-2",
                            label { "Output Mode" }
                            select { class: "border rounded",
                                option { "Objects" }
                                option { "Clusters" }
                                option { "Anomalies" }
                                option { "MapOfCategories" }
                                option { "MapOfModels" }
                                option { "MapOfDistances" }
                                option { "MapOfIdentifiers" }
                            }
                        }
                    }

                    // Second Row
                    div { class: "w-full grid grid-cols-3 gap-x-10 items-center justify-start",

                        // Auto Checkbox
                        div { class: "flex flex-col items-start gap-2",

                            span { class: "flex items-center gap-2",
                                input { r#type: "checkbox", id: "auto" }
                                label { r#for: "auto", "Auto" }
                            }
                            span { class: "flex items-center gap-2",
                                input { r#type: "checkbox", id: "fullimage" }
                                label { r#for: "fullimage", "Full image" }
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

                            span { class: "flex items-center gap-1",
                                input { r#type: "checkbox", id: "skipx" }
                                label { r#for: "skipx", "SkipX" }
                            }

                            span { class: "flex items-center gap-1",
                                input { r#type: "checkbox", id: "skipy" }
                                label { r#for: "skipy", "SkipY" }
                            }
                        }
                    }
                }

                // Right group: radio buttons
                div { class: "flex flex-col justify-start items-center border p-2 rounded shadow ",
                    label { class: "font-bold pb-2", "Show" }

                    div { class: "flex flex-col gap-4",
                        div { class: "flex justify-start items-center gap-2",
                            input {
                                r#type: "radio",
                                name: "showmode",
                                id: "center",
                                checked: true,
                            }
                            label { r#for: "center", "Center" }
                        }
                        div { class: "flex justify-start items-center gap-2",
                            input {
                                r#type: "radio",
                                name: "showmode",
                                id: "box",
                            }
                            label { r#for: "box", "Box" }
                        }
                        div { class: "flex justify-start items-center gap-2",
                            input {
                                r#type: "radio",
                                name: "showmode",
                                id: "shade",
                            }
                            label { r#for: "shade", "Shade" }
                        }
                    }
                }
            }

            // Image preview placeholder
            div { class: "bg-white flex-1 h-[400px] flex items-center justify-center",
                img {
                    src: image_src,
                    alt: "Recongnized image",
                    class: "object-contain max-h-full",
                }
            }
        }
    }
}
