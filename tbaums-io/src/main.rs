#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use manganis;


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
    let _count = use_signal(|| 0);

    rsx! {
    div {
        class: "min-h-screen bg-gray-100 p-10",
        // Main container for the resume
        div {
            class: "max-w-4xl mx-auto bg-white shadow-lg rounded-lg p-6",

            // Header Section (Name & Title)
            div {
                class: "border-b pb-4 mb-4",
                h1 {
                    class: "text-4xl font-bold text-gray-800",
                    "Michael Tanenbaum"
                },
                p {
                    class: "text-xl text-gray-600",
                    "Sales Engineer | Leading technical sales engagements across AI/ML and Cloud Native software"
                }
            }

            // Contact Info Section
            div {
                class: "flex justify-between mb-4 text-gray-700",
                div {
                    p { "Location: San Francisco, CA" }
                    p { "Email: michael.a.tanenbaum [at] gmail.com" }
                }
                div {
                    p {
                        a {
                            href: "https://linkedin.com/in/miketanenbaum",
                            "LinkedIn: linkedin.com/in/miketanenbaum"
                        }
                    }
                    p {
                        a {
                            href: "http://tbaums.io",
                            "Website: tbaums.io"
                        }
                    }
                }
            }
            // Mission Statement
            div {
                class: "border-b pb-4 mb-4",
                p { "Principal Sales/Solutions (pre-sales) Engineer with proven track record of quota attainment and deep hands-on experience across AI/ML/MLOps technologies (ex., Kubeflow), distributed systems and cloud orchestration systems (ex., Kubernetes), and big data technologies (ex., Kafka, Cassandra)."}
            }
            // Experience Section
            div {
                class: "mb-6",
                h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "Experience" }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "Co-founder & CEO" }
                    p { class: "italic text-gray-600", "Mycelial • May 2021 - Sept 2024" }
                    div {
                        class: "my-2",
                        p {
                            "- Co-inventor of Mycelial - an open source, peer-to-peer data movement and inline transformation toolkit used in production by public institutions and enterprises such as the US Navy and Royal Canadian Air Force."
                            a { href: "github.com/mycelial/mycelial", " (See: http://github.com/mycelial/mycelial)"
                            }
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Lead all fundraising efforts, resulting in successful Pre-Seed and Seed funding rounds totaling $3.5MM led by Crane Venture Partners and Accel."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Principal organizational people leader and technical sales GTM leader across all company engagements, including Koch Industries, Lockheed Martin, Walmart, and UK Ministry of Defense."
                        }
                    }
                }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "Principal Solutions Engineer" }
                    p { class: "italic text-gray-600", "Arrikto • Nov 2020 - May 2021" }
                    div {
                        class: "my-2",
                        p {
                            "- Led technical sales engagement for entire company portfolio, implementing enterprise Kubeflow at a fortune 25 energy company, resulting in a multi-year, enterprise-wide sales win with $100MM+ customer economic impact."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Established foundational technical sales documentation, demonstrations, and processes to reduce technical sales lead time and demo failure rates."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Responsible for key technical leadership relationships across company's client portfolio."
                        }
                    }
                }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "Principal Sales Engineer" }
                    p { class: "italic text-gray-600", "D2iQ (formerly Mesosphere) • Sept 2018 - Nov 2020" }
                    div {
                        class: "my-2",
                        p {
                            "- #1 Global Top Seller - FY2020; Global GTM lead for Kaptain (D2iQ distribution of Kubeflow) and Head of Field Big Data & AI/ML practice."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Lead technical sales resource for named strategic accounts across US East, Canada, and Latin America - including JPMorgan Chase, Verizon, NBCUniversal, Prudential, Two Sigma - representing $5MM+ aggregate ACV."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Execute entire technical sales cycle from initial pitch, requirements gathering and scoping, PoC design and installation (on 15,000+ bare metal, on-premise CPU cores), post-sales professional services scoping, and final contract negotiations."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                            "- Successfully delivered closed business across entire D2iQ product portfolio, including: Mesosphere DC/OS, Konvoy (D2iQ Kubernetes distribution), Kommander (D2iQ multi-cluster manager), Mesosphere Data Science Engine, CNCF-certified Kubernetes training, and full suite of professional services."
                        }
                    }
                }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "Co-founder & COO" }
                    p { class: "italic text-gray-600", "Hippo Technologies • Aug 2016 - Apr 2018" }
                    div {
                        class: "my-2",
                        p {
                            "- Drove strategy and execution for digital prescription management and discount medication purchasing application, successfully raising seed funding."
                            a { href: "hellohippo.com", " (See: http://hellohippo.com)"
                            }
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                        "- Hired and managed product and engineering team of 25 across 4 time zones, growing total company headcount from 3 to 55 in 18 months, including 22 software engineers."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                        "Established and managed day-to-day strategic relationships with key partners/vendors, each with $100B+ annual revenue."
                        }
                    }
                }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "Chief of Staff, Technology" }
                    p { class: "italic text-gray-600", "Blink Health • Jan 2016 - Jul 2016" }
                    div {
                        class: "my-2",
                        p {
                            "- Analyzed, architected, and implemented firm-wide data and analytics strategy tracking 250,000+ digital prescription discount card customers."
                            }
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                        "- Hired and trained 5 data analysts, instituting policies and procedures for all financial planning and reporting, marketing ROI analysis, and pricing analysis supporting $10MM+ customer acquisition spend."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                        "- Supported co-founders in PBM contract negotiations and plan implementation through Series A fundraise ($75MM), helping company grow to $50MM+ in annual gross revenue."
                        }
                    }
                }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "Co-founder and CEO" }
                    p { class: "italic text-gray-600", "ConnectCubed • Jun 2010 - Dec 2015" }
                    div {
                        class: "my-2",
                        p {
                            "- Designed, developed, and programmed data gathering, storage, and statistical analysis software for HR executives to assist in data-driven decision-making."
                            }
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                        "- Created and executed 8-stage sales process, communicating via email with 2000+ companies per week, yielding 32% aggregate response rate and 30+ sales meetings with prospective clients per week."
                        }
                    }
                    div {
                        class: "my-2",
                        p {
                        "- Successfully closed 130 VP-level clients with annual contract value ranging up to $45,000."
                        }
                    }

            // Education Section
            div {
                class: "mb-6 mt-10",
                h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "Education" }
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "M.A. in International Finance and China Studies" }
                    p { class: "italic text-gray-600", "Johns Hopkins School of Advanced International Studies (SAIS) • Graduated 2010" }
                }
            }

            div {
                class: "mb-6",
                div {
                    class: "mb-4",
                    h3 { class: "text-lg font-bold", "B.A. in International Relations" }
                    p { class: "italic text-gray-600", "The Johns Hopkins University • Graduated 2009" }
                }
            }
            // Skills Section
            div {
                h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "Skills & Certifications" }
                ul {
                    class: "list-disc pl-5 text-gray-700",
                    li { "Programming Languages: Rust, SQL, Python, JavaScript" }
                    li { "Data Infrastructure: Kafka, Cassandra, Postgres, Kubeflow, SQLite" }
                    li { "Cloud & Orchestration: Kubernetes (Certified Kubernetes Administrator), Docker, Terraform" }
                }
            }
        }
    }
    }
}
