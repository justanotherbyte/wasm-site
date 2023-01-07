use dioxus::prelude::*;
use chrono::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use dioxus_markdown::Markdown;
use dioxus_router::Link;

pub fn HomePage(cx: Scope) -> Element {
    let now = chrono::Utc::now();
    let birthday = chrono::Utc.with_ymd_and_hms(2007, 03, 23, 7, 0, 0).unwrap();
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
                        to: "/notebooks",
                        class: "inline-flex items-center justify-center py-3 mr-3 text-base font-medium text-center text-white rounded-lg hover:text-rose-400 duration-200",
                        "Notebooks"
                    },
                    Link {
                        to: "/blog",
                        class: "inline-flex items-center justify-center px-5 py-3 mr-3 text-base font-medium text-center text-white rounded-lg hover:text-rose-400 duration-200",
                        "Blog"
                    },
                    a {
                        href: "https://github.com/justanotherbyte",
                        class: "inline-flex items-center justify-center py-3 mr-3 text-base font-medium text-center text-white rounded-lg hover:text-rose-400 duration-200",
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
                }
                div {
                    class: "grid grid-cols-2 gap-4 mt-8",
                    img {
                        class: "w-full rounded-lg hover:-translate-y-6 duration-300",
                        src: "/images/img328.jpg"
                    }
                    img {
                        class: "mt-4 w-full lg:mt-10 rounded-lg hover:translate-y-6 duration-300",
                        src: "/images/img321.jpg"
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn BlogPostCard(cx: Scope, title: String, content: String, image: String, slug: String) -> Element {
    cx.render(rsx!{
        div {
            class: "max-w-sm bg-white rounded-lg shadow-md bg-zinc-800 p-3 mb-3",
            a {
                class: "",
                img {
                    class: "rounded-lg",
                    src: "{image}"
                }
            }
            div {
                class: "p-3 mt-3",
                a {
                    h5 {
                        class: "mb-2 text-2xl font-bold tracking-tight text-white",
                        "{title}"
                    }
                }
                p {
                    class: "mb-3 font-normal text-zinc-400",
                    "{content}"
                }
                Link {
                    to: "/blog/{slug}",
                    class: "inline-flex items-center text-sm font-medium text-center text-sky-400 hover:text-rose-400 duration-200",
                    "Read more ➜"
                }
            }
        }
    })
}

#[derive(Deserialize)]
pub struct BlogPostModel {
    pub slug: String,
    pub content: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub created_at: String,
    pub category: String
}

impl BlogPostModel {
    fn loading() -> Self {
        Self { 
            slug: "".into(),
            content: "".into(),
            name: "Loading...".into(),
            description: "Hang on tight! Just warming up our gears here!".into(),
            created_at: "0000-00-0".into(),
            image: "/loading.svg".into(),
            category: "Loading...".into()
         }
    }
}

#[derive(Deserialize, Clone)]
pub struct RepoNotebook {
    pub name: String,
    pub html_url: String
}

impl RepoNotebook {
    fn loading() -> Self {
        Self {
            name: "Kind of cold in here, just warming up!".into(),
            html_url: "/notebooks".into()
        }
    }
}

fn fetch_posts(cx: &Scope, state: &UseState<Vec<BlogPostModel>>) {
    let state = state.to_owned();
    cx.spawn(async move {
        let resp = Request::new("https://blogserver-quiktea.vercel.app/posts")
            .send()
            .await
            .unwrap()
            .json::<Vec<BlogPostModel>>()
            .await
            .unwrap();

        state.set(resp);
    });
}

// https://api.github.com/repos/OWNER/REPO/contents/PATH
fn fetch_notebooks(cx: &Scope, state: &UseState<Vec<RepoNotebook>>) {
    let state = state.to_owned();
    cx.spawn(async move {
        let resp = Request::new("https://api.github.com/repos/justanotherbyte/notebooks/contents/")
            .send()
            .await
            .unwrap()
            .json::<Vec<RepoNotebook>>()
            .await
            .unwrap();

        state.set(resp);
    });
}

fn fetch_post(cx: &Scope<BlogPostPageProps>, state: &UseState<BlogPostModel>, slug: String) {
    let state = state.to_owned();
    let url = format!("https://blogserver-quiktea.vercel.app/post/{}", slug);
    cx.spawn(async move {
        let resp = Request::new(url.as_str())
            .send()
            .await
            .unwrap()
            .json::<BlogPostModel>()
            .await
            .unwrap();

        state.set(resp);
    });
}

pub fn BlogPage(cx: Scope) -> Element {
    let post_state: &UseState<Vec<BlogPostModel>> = use_state(&cx, || vec![BlogPostModel::loading()]);
    cx.use_hook(|_| fetch_posts(&cx, &post_state));

    let posts_parsed = post_state.get();

    let mut counter = 0;

    cx.render(rsx!{
        section {
            class: "bg-transparent",
            posts_parsed.iter().rev().map(|model| {
                if counter == 0 {
                    counter += 1;
                    rsx!{
                        div {
                            class: "grid max-w-screen-xl px-4 py-8 mx-auto lg:gap-8 xl:gap-0 lg:py-16 lg:grid-cols-12",
                            div {
                                class: "mr-auto place-self-center lg:col-span-7",
                                h1 {
                                    class: "max-w-2xl mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl text-white",
                                    "{model.name}"
                                }
                                p {
                                    class: "max-w-2xl mb-6 font-light lg:mb-8 md:text-lg lg:text-xl text-zinc-400",
                                    p {
                                        "{model.description}"
                                    }
                                }
                                Link {
                                    to: "/blog/{model.slug}",
                                    class: "inline-flex items-center justify-center py-3 mr-3 text-base font-medium text-center text-sky-400 rounded-lg hover:text-rose-400 duration-200",
                                    "Read More ➜"
                                },
                            }
                            div {
                                class: "hidden lg:mt-0 lg:col-span-5 lg:flex",
                                img {
                                    class: "rounded-lg object-cover",
                                    src: "{model.image}"
                                }
                            }
                        }
                    }
                } else {
                    rsx! { p{} }
                }
            })
        }

        div {
            class: "container mx-auto grid sm:grid-cols-1 md:grid-cols-3 space-x-3 p-3",
            posts_parsed.iter().map(|model| rsx!{
                BlogPostCard { title: model.name.clone(), content: model.description.clone(), image: model.image.clone(), slug: model.slug.clone() }
            })
        }
    
    })
}

#[inline_props]
pub fn BlogPostPage(cx: Scope) -> Element {
    let slug = use_route(&cx);
    let slug = slug.last_segment().unwrap().to_string();
    let post_state = use_state(&cx, BlogPostModel::loading);
    cx.use_hook(|_| fetch_post(&cx, post_state, slug.to_owned()));

    let post = post_state.get();

    cx.render(rsx!{
        section {
            class: "container mx-auto mt-10",
            p {
                class: "text-zinc-400 text-center mb-3",
                "{post.category}"
            }
            h2 {
                class: "text-4xl text-white font-extrabold mb-3 text-center",
                "{post.name}"
            }
            h6 {
                class: "text-xl font-medium text-zinc-400 text-center mb-3",
                "By Viswa Marepalli"
            }
            img {
                class: "w-3/4 mx-auto rounded-lg h-96 object-cover object-bottom",
                src: "{post.image}"
            }
            Markdown {
                class: "text-zinc-300 mx-auto text-left mt-14 leading-relaxed max-w-5xl m-3 rounded-md duration-300 p-3",
                id: "post-content",
                content: post.content.as_str()
            }
        }
    })
}

#[inline_props]
pub fn NotebookCard(cx: Scope, name: String, hide: bool, url: String) -> Element {
    if hide.to_owned() {
        None
    } else {
        let split: Vec<&str> = name.split(".").collect();
        let ext;
        if split.len() == 1 {
            ext = "folder";
        } else {
            ext = split[1];
        }
        cx.render(rsx!{
            a {
                href: "{url}",
                class: "flex border border-gray-300 text-white p-3 w-full text-left first:rounded-t-md last:rounded-b-md hover:text-black hover:bg-gray-300 duration-300",
                img {
                    src: "/images/{ext}.svg",
                    class: "w-7 h-auto mr-1"
                }
                "{name}"
            }
        })
    }
}

pub fn NotebooksPage(cx: Scope) -> Element {
    let book_state: &UseState<Vec<RepoNotebook>> = use_state(&cx, || vec![RepoNotebook::loading()]);
    cx.use_hook(|_| fetch_notebooks(&cx, &book_state));

    let books = book_state.get();
    let query = use_state(&cx, String::new);

    cx.render(rsx!{
        div {
            class: "container mx-auto",
            h1 {
                class: "text-3xl text-white font-bold px-5 mt-10",
                "Notebooks"
            },
            p {
                class: "text-gray-400 px-5 mt-3 font-medium",
                "A catalogue of my Machine Learning and Data Science notebooks. Synced with my "
                a {
                    class: "text-sky-500",
                    href: "https://github.com/justanotherbyte/notebooks",
                    "notebooks "
                }
                "repository."
            }
            div {
                class: "px-5 mt-3",
                input {
                    class: "bg-zinc-800 py-2 rounded-md text-gray-300 px-2 ring-0 ring-sky-500 focus:ring focus:outline-none border-none",
                    oninput: |evt| query.set(evt.value.clone()),
                    placeholder: "Search"
                }
            }
            ul {
                class: "mt-8 px-5",
                books.iter().map(|book| {
                    if query.get().is_empty() {
                        rsx! {
                            NotebookCard { name: book.name.clone(), hide: false, url: book.html_url.clone() }
                        }
                    } else {
                        if book.name.starts_with(query.get().as_str()) {
                            rsx! {
                                NotebookCard {
                                    name: book.name.clone(),
                                    hide: false,
                                    url: book.html_url.clone()
                                }
                            }
                        } else {
                            rsx! {
                                NotebookCard {
                                    name: "".into(),
                                    hide: true,
                                    url: book.html_url.clone()
                                }
                            }
                        }
                    }
                })
            }
        }
    })
}