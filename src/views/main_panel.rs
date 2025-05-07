use base64::engine::general_purpose;
use base64::Engine;
use dioxus::prelude::*;
use rfd::FileDialog;
use std::fs;

use crate::components::learn::LearnSection;
use crate::components::reco::RecoSection;
use crate::views::footer::Footer;

pub fn MainPanel() -> Element {
    // Make signal mutable
    let mut image_data_url = use_signal(|| None::<String>);

    let mut selected = use_signal(|| "".to_string());

    let pick_image = move |_| {
        if let Some(path) = FileDialog::new().add_filter("Image", &["png"]).pick_file() {
            if let Ok(bytes) = fs::read(&path) {
                let mime = match path.extension().and_then(|e| e.to_str()) {
                    Some("png") => "image/png",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("webp") => "image/webp",
                    _ => "application/octet-stream",
                };
                let encoded = general_purpose::STANDARD.encode(bytes);
                let data_url = format!("data:{};base64,{}", mime, encoded);
                // Update the signal value
                image_data_url.set(Some(data_url));
                selected.set(path.display().to_string());
            }
        }
    };

    rsx! {
        main { class: "w-full flex flex-row gap-2 flex-1 bg-gray-400 p-4 rounded shadow",
            LearnSection { image_data_url: image_data_url.clone() }
            RecoSection { image_data_url: image_data_url.clone() }
        }
        Footer { pick_image, selected }
    }
}
