use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_server_future(move || favorite_dogs())?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for url in favorites.value().cloned().unwrap().unwrap().into_iter().rev() {
                    FavoriteDog { url }
                }
            }
        }
    }
}

#[component]
fn FavoriteDog(url: ReadOnlySignal<String>) -> Element {
    rsx! {
        div { class: "favorite-dog",
            img { src: "{url}" }
        }
    }
}

#[server]
async fn favorite_dogs() -> Result<Vec<String>, ServerFnError> {
    Ok(std::fs::read_to_string("dogs.txt")
        .unwrap_or_default()
        .lines()
        .map(|f| f.to_string())
        .collect())
}
