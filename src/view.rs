use dioxus::prelude::*;

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        #[derive(serde::Deserialize)]
        struct DogApi {
            message: String,
        }

        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let save = move |_| async move {
        let current = img_src.value().cloned().unwrap();
        img_src.restart();
        save_dog(current).await.unwrap();
    };

    let skip = move |_| {
        img_src.restart();
    };

    rsx! {
        div { id: "dogview",
            img {
                id: "dogimg",
                src: "{img_src.value().cloned().unwrap_or_default()}",
            }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    _ = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap()
        .write_fmt(format_args!("{image}\n"));

    Ok(())
}
