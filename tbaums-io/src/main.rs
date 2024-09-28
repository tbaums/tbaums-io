#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
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

    rsx! {
        div {
            class: "pt-2 md:pt-5 lg:pt-8 bg-lavender",
            div {
                id: "container",
                class: "container mx-auto bg-plum/50 md:rounded-lg md:pb-3 pb-2 md:pr-2 md:pt-4 pt-2",
                div {
                    class: "grid grid-cols-1 md:grid-cols-[1fr_auto] gap-2 md:gap-4",
                    div {
                        style: "box-shadow: 0px 0px 5px lavender;",
                        class: "opacity-75 text-4xl border-b-black-violet border-b-0 md:border-b-2 text-black-violet pl-1 pb-2 pt-1 md:ml-3 md:pl-3 md:pb-4 md:pt-2 font-bold bg-lavender",
                        "Michael Tanenbaum"
                    }
                    div {
                        class: "text-black-violet border-b-black-violet border-b-0 md:border-b-2 bg-lavender opacity-75 text-center grid place-items-center text-4xl",
                        Link {
                            to: Route::Home {  },
                            ">"
                        }
                    }   
                    div {
                        class: "md:border-l-2 md:border-b-2 md:border-black-violet bg-lavender opacity-75 pl-1 pb-2 pt-1 md:ml-3 md:pl-3 md:pb-4 md:pt-2",
                        "Down low!"
                    }
                    div {
                        class: "md:border-b-2 md:border-black-violet bg-lavender opacity-75 pl-1 pb-2 pt-1 md:ml-0 md:pl-0 md:pb-4 md:pt-2",
                        "Go to blog"
                    }
                }
            }
        }
    }
}
