use dioxus::prelude::*;

use crate::components::learn::LearnSection;
use crate::components::reco::RecoSection;
use crate::views::footer::Footer;

pub fn MainPanel() -> Element {
    rsx! {
        main {
            class: "w-full flex flex-row gap-2 flex-1 bg-gray-200 p-4 rounded shadow",
            LearnSection {}
            RecoSection {}
        }
        Footer {}
    }
}
