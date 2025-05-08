use dioxus::{html::div, prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct RecoSectionProps {
    image_data_url: Signal<Option<String>>,
}

pub fn RecoSection(props: RecoSectionProps) -> Element {
    let image_src = props.image_data_url.read().clone().unwrap_or(
        "/home/nitin/Nitin_Personal/Rust/image_knowledge_builder/assets/lens.svg".to_string(),
    );

    rsx! {
        section { class: "w-full flex flex-col gap-6 p-4 text-sm",

            // Top bar: Learn button, dropdowns, select image, auto checkbox
            div { class: "flex flex-wrap  justify-start gap-10 h-28",

                div { class: "flex flex-col gap-2",

                    div { class: "flex flex-wrap gap-10 justify-start items-center",

                        div { class: "flex justify-start items-center gap-2",
                            button { class: "bg-white p-3 rounded border shadow", "Reco" }
                        }

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

                    div { class: "flex flex-wrap gap-10 justify-start items-center",

                        div { class: "flex justify-start items-center gap-2",
                            input { r#type: "checkbox", id: "auto" }
                            label { r#for: "auto", "Auto" }
                        }

                        div { class: "flex justify-start items-center gap-2",
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

                    div { class: "flex flex-wrap gap-10 justify-start items-center",

                        div { class: "flex justify-start items-center gap-2",
                            input { r#type: "checkbox", id: "fullimage" }
                            label { r#for: "fullimage", "Full image" }
                        }
                    }
                }

                // Right group: radio buttons
                div { class: "flex flex-col justify-start items-center border p-2 rounded shadow ",
                    label { class: "font-bold pb-2", "Show" }

                    div { class: "flex flex-col gap-1",
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
            div { class: "bg-white flex-1 h-[400px] mt-4 flex items-center justify-center",
                img {
                    src: image_src,
                    alt: "Recongnized image",
                    class: "object-contain max-h-full",
                }
            }
        }
    }
}
