#![allow(non_snake_case)]

mod pages;

use dioxus::{
    prelude::*,
    router::{Route, Router}
};

use pages::{HomePage, BlogPage, BlogPostPage, NotebooksPage, JourneyPage};

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        Router {
            Route {
                to: "/",
                HomePage {}
            },
            Route {
                to: "/notebooks",
                NotebooksPage {}
            },
            Route {
                to: "/journey",
                JourneyPage {}
            },
            Route {
                to: "/blog",
                BlogPage {}
            },
            Route {
                to: "/blog/:id",
                BlogPostPage {}
            }
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx!{

        nav {
            class: "px-2 sm:px-4 py-2.5 rounded bg-transparent",
            div {
                class: "container flex flex-wrap items-center justify-between mx-auto",
                a {
                    class: "flex items-center",
                    href: "/",
                    h2 {
                        class: "self-center text-xl font-semibold whitespace-nowrap text-white",
                        "Viswa"
                    }
                },
                ul {
                    class: "flex flex-col py-1 px-3 mt-4 rounded-lg md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-transparent bg-rose-400",
                    li {
                        a {
                            class: "block pl-3 pr-4 rounded md:p-0 text-white md:hover:text-white hover:bg-gray-700 hover:text-white md:hover:bg-transparent",
                            "Home"
                        }
                    }
                }
            }
        }
    })
}