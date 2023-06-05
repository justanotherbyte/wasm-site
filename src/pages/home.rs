use chrono::prelude::*;
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn HomePage(cx: Scope) -> Element {
    // spa hack
    let window = web_sys::window();
    if let Some(window) = window {
        let storage = window.local_storage();
        if let Ok(Some(storage)) = storage {
            crate::log("Storage exists, checking for path");
            let path = storage.get("path");
            if let Ok(Some(path)) = path {
                // if we find a path, create a new Link and click on it
                // dioxus handles the rest
                return cx.render(rsx!{
                    Link {
                        to: "{path}",
                        id: "spa-hack",
                        "Click if you're not redirected"
                    }
                })
            }
        }
    };
    let now = chrono::Utc::now();
    let birthday = chrono::Utc.with_ymd_and_hms(2007, 3, 23, 7, 0, 0).unwrap();
    let years_since = now.years_since(birthday).unwrap();
    cx.render(rsx!{
        section {
            class: "bg-transparent",
            div {
                class: "grid max-w-screen-xl px-4 py-8 mx-auto lg:gap-8 xl:gap-0 lg:py-16 lg:grid-cols-12",
                div {
                    class: "mr-auto place-self-center lg:col-span-7",
                    h1 {
                        class: "max-w-2xl mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl text-white",
                        "Hi. I'm Viswa"
                    }
                    p {
                        class: "max-w-2xl mb-6 font-light text-zinc-500 lg:mb-8 md:text-lg lg:text-xl dark:text-zinc-400",
                        p {
                            "Nice to meet you! I'm currently a student in England. I love working on projects related to",
                            a { 
                              class: "text-rose-400 hover:animate-pulse",
                              href: "https://en.wikipedia.org/wiki/Machine_learning",
                              " Machine Learning"
                            },
                            ", "
                            a {
                                class: "text-sky-400 hover:animate-pulse",
                                href: "https://en.wikipedia.org/wiki/Mathematics",
                                " Mathematics"
                            },
                            " and my video game ",
                            a {
                                class: "text-emerald-400 hover:animate-pulse",
                                href: "https://github.com/justanotherbyte/nova",
                                "Nova"
                            },
                            ", which you should totally try out by the way 😉",
                        }
                    }
                    Link {
                        to: "https://www.kaggle.com/justanotherbyte",
                        class: "inline-flex items-center justify-center py-3 mr-3 text-base font-medium text-center text-white rounded-lg hover:text-sky-400 duration-200",
                        "Kaggle"
                    },
                    Link {
                        to: "/blog",
                        class: "inline-flex items-center justify-center px-5 py-3 mr-3 text-base font-medium text-center text-white rounded-lg hover:text-rose-400 duration-200",
                        "Blog"
                    },
                    a {
                        href: "https://github.com/justanotherbyte",
                        class: "inline-flex items-center justify-center py-3 mr-3 text-base font-medium text-center text-white rounded-lg hover:text-emerald-400 duration-200",
                        "GitHub"
                    }
                }
                div {
                    class: "hidden lg:mt-0 lg:col-span-5 lg:flex",
                    img {
                        src: "/images/avatar.png",
                        class: "rounded-full"
                    }
                }
            }
        }

        section {
            id: "about",
            class: "bg-transparent",
            div {
                class: "gap-16 items-center py-8 px-4 mx-auto max-w-screen-xl lg:grid lg:grid-cols-2 lg:py-16 lg:px-6",
                div {
                    class: "font-light text-zinc-500 sm:text-lg dark:text-zinc-400",
                    h2 {
                        class: "mb-4 text-4xl tracking-tight font-extrabold text-white",
                        "About Me"
                    }
                    p {
                        class: "mb-4",
                        "I'm a {years_since} year old currently studying in England. I have a huge interest in Machine Learning, Mathematics and Computer Science as a whole"
                        Link {
                            to: "/upload",
                            "."
                        }
                    }
                    p {
                        "I'll be beginning my journey on the ",
                        a {
                            class: "text-sky-400 hover:animate-pulse",
                            href: "https://ibo.org/programmes/diploma-programme/",
                            "International Baccalaureate Diploma "
                        }
                        "programme soon (I may already be on it, I've just forgotten to update this page). I'd like to study Computer Science at Cambridge or Imperial in the future :D"
                    }
                    div {
                        class: "group mt-3 max-w-fit border border-white group-hover:text-black hover:bg-white duration-300 rounded-md",
                        Link {
                            to: "/journey",
                            class: "p-3 group-hover:text-black text-white",
                            "Read More"
                        }
                    }
                    
                }
                div {
                    class: "grid grid-cols-2 gap-4 mt-8",
                    img {
                        class: "w-full rounded-lg hover:-translate-y-6 duration-300",
                        src: "/images/homepic1.png"
                    }
                    img {
                        class: "mt-4 w-full lg:mt-10 rounded-lg hover:translate-y-6 duration-300",
                        src: "/images/homepic2.png"
                    }
                }
            }
        }
    })
}
