use dioxus::prelude::*;

pub fn RecoSection() -> Element {
    rsx! {
        section {
            class: "w-full flex flex-col gap-4 bg-gray-300 p-4 rounded shadow text-sm",

            // Top bar: Learn button, dropdowns, select image, auto checkbox
            div {
                class: "flex items-center justify-start gap-2",

                div {
                    class: "grid grid-cols-2 gap-2 ",

                    div {
                        class:"flex justify-start items-center gap-2 ",
                        button {
                            class: "bg-white p-3 rounded border shadow",
                            "Reco"
                        }
                    }

                    div {
                        class:"flex flex-col gap-2 ",
                        label { "Output Mode" }
                        select {
                            class: "border  rounded",
                            option { "Objects" }
                            option { "Clusters" }
                            option { "Anomalies" }
                            option { "MapOfCategories" }
                            option { "MapOfModels" }
                            option { "MapOfDistances" }
                            option { "MapOfIdentifiers" }
                        }
                    }

                    div {
                        class:"flex justify-start items-center gap-2",
                        input { r#type: "checkbox", id: "auto" }
                        label { r#for: "auto", "Auto" }

                    }

                    div {
                        class:"flex flex-row gap-2 justify-start items-center",
                        label { "StepXY" }
                        input { r#type: "number", value: "16", class: "w-12 p-1 border rounded" }
                        input { r#type: "number", value: "16", class: "w-12 p-1 border rounded" }

                        span {
                            class:"flex flex-row gap-2 justify-start items-center",
                             input { r#type: "checkbox", id: "skipx" }
                             label { r#for: "skipx", "SkipX" }
                        }

                        span {
                            class:"flex flex-row gap-2 justify-start items-center",
                             input { r#type: "checkbox", id: "skipy" }
                             label { r#for: "skipy", "SkipY" }
                        }
                    }

                }


                div {
                       class: "flex flex-col justify-start items-start gap-1 border p-2 rounded  shadow",
                       label { class: "font-bold", "Show" }

                       div {
                           class: "flex flex-col gap-1",
                           div {
                               class:"flex justify-start items-center gap-2",
                               input { r#type: "radio", name: "showmode", id: "center", checked: true }
                               label { r#for: "center", "Center" }
                           }
                           div {
                               class:"flex justify-start items-center gap-2",
                               input { r#type: "radio", name: "showmode", id: "box" }
                               label { r#for: "box", "Box" }
                           }
                           div {
                               class:"flex justify-start items-center gap-2",
                               input { r#type: "radio", name: "showmode", id: "shade" }
                               label { r#for: "shade", "Shade" }
                           }
                       }
                   }



            }


            // Image preview placeholder
            div {
                class: "bg-white flex-1 h-[400px] mt-4 flex items-center justify-center",
                img {
                    src: "/path/to/learn.png", // Replace with actual image URL or logic
                    class: "object-contain max-h-full"
                }
            }
        }
    }
}
