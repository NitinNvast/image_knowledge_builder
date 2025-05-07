use dioxus::prelude::*;

pub fn LearnSection() -> Element {
    rsx! {
        section {
            class: "w-full flex flex-col gap-4 bg-gray-300 p-4 rounded shadow text-sm",

            // Top bar: Learn button, dropdowns, select image, auto checkbox
            div {
                class: "flex items-center justify-between",

                div {
                    class: "grid grid-cols-3 gap-2 justify-start items-center",

                    div {
                        class:"flex justify-start items-center gap-2",
                        button {
                            class: "bg-white p-3 rounded border shadow",
                            "Learn"
                        }
                    }

                    div {
                        class:" flex flex-col gap-2 ",
                        label { "Learn Mode" }
                        select {
                            class: "border  rounded",
                            option { "Codebook" }
                            option { "Annotations" }
                        }
                    }

                    div {
                        class:" flex flex-col gap-2",
                        label { "Categories" }
                        select {
                            class: "border rounded",
                            option { "Increment" }
                            option { "Constant" }
                            option { "MaxDeltaAmplitude" }
                            option { "AvgAmplitude" }
                            option { "InputIndex" }
                        }
                    }

                    div {
                        class:" flex justify-start items-center gap-2",
                        input { r#type: "checkbox", id: "auto" }
                        label { r#for: "auto", "Auto" }
                    }

                    div {
                        class:" flex justify-start items-center gap-2",
                        button {
                            class: "co bg-white px-3 py-1 rounded border shadow",
                            "Select full image"
                        }
                    }

                    div {
                        class:" flex justify-start items-center gap-2",
                        label { "StepXY" }
                        input { r#type: "number", value: "16", class: "w-12 p-1 border rounded" }
                        input { r#type: "number", value: "16", class: "w-12 p-1 border rounded" }
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
