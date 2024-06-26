#![allow(non_snake_case)]
mod ui_components;

use dioxus::prelude::*;
use tracing::{info, Level};

use crate::ui_components::button::Button;

const _STYLE: &str = manganis::mg!(file("./public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            class: "container mx-auto px-4",
            h1 {
                class:"text-3xl font-bold ",
                "High-Five counter: {count}" }
            Button{
                class:"btn btn-primary",
                "Custom Button"
            }
            button { class: "btn btn-primary", onclick: move |_| count += 1, "Up high!" }
            button { class: "btn btn-secondary", onclick: move |_| count -= 1, "Down low!" }
            button {
                class: "btn btn-accent",
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p {
                class: "font-bold",
                 "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
