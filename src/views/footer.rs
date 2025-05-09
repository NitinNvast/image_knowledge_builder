use dioxus::prelude::*;

pub fn Footer() -> Element {
    rsx! {
        div { class: "bg-gray-100 text-xs p-1 flex flex-col space-y-1",
            div { class: "flex justify-start w-full gap-2",
                span { "NN capacity:8000" }
                span { "Neurons:0" }
                span { "Timings OFF" }
                span { class: "text-yellow-600 font-bold", "Loading default project file..." }
            }
            div { class: "flex flex-wrap gap-2",
                span { class: "text-blue-700", "Project: default.csp" }
                span { "ImageFile" }
                span { "ROI: 16x16, Subsample, with blocks 1x1, Minif: 2, Maxif: 16384" }
                span { class: "text-blue-700", "Search Area: [0, 0, 0, 0]" }
            }
        }
    }
}
