use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod views;
use views::footer::Footer;
use views::main_panel::MainPanel;
use views::navbar::Navbar;

mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet {  href: TAILWIND_CSS }


        div {
        class: "h-screen flex flex-col",
        Navbar {}
        MainPanel {}
        // Footer {}
        }

    }
}
