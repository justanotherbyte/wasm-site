use dioxus::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

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