mod favorites;
mod nav;
mod view;

use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(nav::HeaderFooter)]
    #[route("/", view::DogView)]
    DogView,

    #[route("/favorites", favorites::Favorites)]
    Favorites,
}

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Stylesheet { href: asset!("/assets/main.css") }
            Router::<Route> {}
        }
    });
}
