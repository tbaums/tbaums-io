#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedinIn};
use dioxus_free_icons::Icon;
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
    let count = use_signal(|| 0);

    rsx! {
        div {
            class: "pt-2 md:pt-5 lg:pt-8 bg-pale-violet-red/35 text-black-violet min-h-screen",
            div {
                id: "container",
                class: "container mx-auto bg-plum md:rounded-lg md:pb-3 mb-5 pb-2 md:pr-2 md:pt-4 pt-2",
                div {
                    id: "grid",
                    class: "grid grid-cols-1 md:grid-cols-[1fr_auto] gap-2 md:gap-x-2 md:gap-y-4",
                    div {
                        id: "header",
                        class: "opacity-75  border-black-violet border-b-0 md:border-b-2 pl-1 pb-2 pt-1 md:ml-3 md:pl-3 md:pb-4 md:pt-2 font-bold bg-lavender",
                        h1 {
                            class: "text-2xl md:text-4xl mb-1",
                            "Michael Tanenbaum" 
                        }
                        h3 {
                            class: "text-xl md:text-2xl",
                            "Senior Sales/Solutions Engineer"
                        }
                        h3 {
                            class: "text-lg md:text-xl",
                            "San Francisco, CA"
                        }
                        div {
                            class: "text-sm my-3 md:w-5/6",
                            "Dynamic, engaging, and hands-on technical Sales Engineer with a proven 15-year track record of quota attainment and beyond. I pride myself on developing close relationships with customers and serving as a trusted advisor year after year. My greatest thrill is watching customers discover how to solve their business challenges with technology."
                        }
                        div {
                            class: "text-sm my-3 md:w-5/6",
                            "Since 2018, my focus has been on the Big Data and AI/ML ecosystem running on Kubernetes and Docker (DataOps/DevOps/MLOps). I have worked with customers to deploy and manage large-scale distributed systems on-premises and in the Cloud, including Kafka, Cassandra, and Kubeflow. I am also a Certified Kubernetes Administrator (CKA)."
                        }
                    }
                    div {
                        id: "links",
                        class: "border-b-black-violet p-1 md:p-2 border-b-0 md:border-b-2 bg-lavender opacity-75 text-center grid gap-1 md:gap-1 grid-flow-col md:grid-flow-row md:grid-cols-2 place-items-center text-4xl",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "black",
                            icon: FaGithub,
                        }
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "black",
                            icon: FaLinkedinIn,
                        }
                    }
                    div {
                        class: "md:border-l-2 md:border-b-2 md:border-black-violet bg-lavender opacity-75 pl-1 pb-2 pt-1 md:ml-3 md:pl-3 md:pb-4 md:pt-2",
                        div {
                            h1 {
                                class: "text-lg font-bold md:text-2xl mb-1",
                                "Co-Founder & CEO"
                            }
                            h2 {
                                class: "text-md font-semibold md:text-xl mb-1",
                                "Mycelial, Inc."
                            }
                            ul {
                                class: "md:w-5/6",
                                li {
                                    class: "text-sm md:text-md",
                                    "Co-inventor of Mycelial - an open source, peer-to-peer data movement and inline transformation toolkit used in production by public institutions and enterprises such as the US Navy and Royal Canadian Air Force. (github.com/mycelial/mycelial)"
                                }
                                li {
                                    class: "text-sm",
                                    "Lead all fundraising efforts, resulting in successful Pre-Seed and Seed funding rounds totaling $3.5MM."
                                }
                                li {
                                    class: "text-sm",
                                    "Principal organizational people leader and technical sales GTM leader across all company engagements."
                                }
                            }
                        }
                    }
                    }
                    div {
                        class: "md:border-b-2 md:border-black-violet bg-lavender opacity-75 pl-1 pb-2 pt-1 md:ml-0 md:px-2 md:pb-4 md:pt-2 font-bold",
                        "Jan 2020 - Sept 2024"
                    }
                }
            }
        }
    }
