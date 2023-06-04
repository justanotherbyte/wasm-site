use dioxus::prelude::*;

use crate::markdown::markdown_to_html;

pub fn run_markdown_parse(cx: Scope, state: UseState<String>, md: String) {
    cx.spawn(async move {
        let html = markdown_to_html(&md).await;
        state.set(html)
    })
}
pub fn highlight_code() {
    crate::log("highlighting code");
    js_sys::Function::new_no_args(&String::from("highlightCode();"))
        .call0(&wasm_bindgen::JsValue::NULL)
        .unwrap();
}

pub fn pull_post(
    cx: Scope,
    slug: &str,
    md_content: UseState<String>,
    name: UseState<String>,
    description: UseState<String>,
    image: UseState<String>,
    category: UseState<String>
) {
    let url = format!("https://site-functions.vercel.app/post/{}", slug);
    cx.spawn(async move {
        let resp = gloo_net::http::Request::new(url.as_str())
            .send()
            .await
            .unwrap()
            .json::<crate::pages::blog::BlogPostModel>()
            .await
            .unwrap();

        crate::log(&resp.content);

        md_content.set(resp.content.clone());
        name.set(resp.name.clone());
        description.set(resp.description.clone());
        category.set(resp.category.clone());
        image.set(resp.image);
    });
}

pub fn publish_post(
    cx: Scope,
    slug: &str,
    content: &str,
    name: &str,
    image: &str,
    description: &str,
    category: &str,
    password: String,
) {
    let payload = serde_json::json!({
        "slug": slug,
        "content": content,
        "name": name,
        "image": image,
        "description": description,
        "category": category
    });

    let url = "https://site-functions.vercel.app/upsert-post";
    cx.spawn(async move {
        gloo_net::http::Request::post(url)
            .json(&payload)
            .unwrap()
            .header("Authorization", &password)
            .send()
            .await
            .unwrap();
    })
}

pub fn BlogEditPage(cx: Scope) -> Element {
    let slug_state = use_state(&cx, String::new);
    let password_state = use_state(&cx, String::new);
    let md_content = use_state(&cx, String::new);
    let input_content = use_state(&cx, String::new);

    let name = use_state(&cx, String::new);
    let description = use_state(&cx, String::new);
    let image = use_state(&cx, String::new);
    let category = use_state(&cx, String::new);

    let message = use_state(&cx, String::new);

    cx.render(rsx! {
        section {
            h1 {
                class: "text-center text-zinc-300 text-2xl mt-5 font-semibold",
                "Blog Editor"
            }
            p {
                class: "text-center text-zinc-400 text-lg mt-1 font-medium",
                "Yes this is public, I'm aware. No, does this not mean anyone can post a blog, you need the password. No, this isn't checked client-side."
            }
        }
        div {
            class: "grid grid-cols-5 grid-rows-2 container px-24 mx-auto justify-items-center mt-5 space-y-3",
            div {
                input {
                    class: "rounded-md bg-zinc-800 p-1 border-2 border-zinc-700 px-2 text-zinc-200 focus:outline-none",
                    placeholder: "Enter Slug",
                    oninput: |ev| slug_state.set(ev.value.clone())
                },
            }
            div {
                button {
                    onclick: |_| {
                        pull_post(
                            cx,
                            slug_state.get(),
                            input_content.to_owned(),
                            name.to_owned(),
                            description.to_owned(),
                            image.to_owned(),
                            category.to_owned()
                        )
                    },
                    class: "rounded-md bg-zinc-800 p-1 border-2 border-zinc-700 text-zinc-200 hover:bg-zinc-700 duration-200",
                    "Pull Blog"
                }
            }
            div {
                button {
                    onclick: |_| {
                        publish_post(
                            cx,
                            slug_state.get(),
                            input_content.get(),
                            name.get(),
                            image.get(),
                            description.get(),
                            category.get(),
                            password_state.get().to_owned()
                        )
                    },
                    class: "rounded-md bg-zinc-800 p-1 border-2 border-zinc-700 text-zinc-200 hover:bg-zinc-700 duration-200",
                    "Publish Blog"
                }
            }
            div {
                button {
                    class: "rounded-md bg-zinc-800 p-1 border-2 border-zinc-700 text-zinc-200 hover:bg-zinc-700 duration-200",
                    onclick: |_| highlight_code(),
                    "Highlight Code"
                }
            }
            div {
                input {
                    class: "rounded-md bg-zinc-800 p-1 border-2 border-zinc-700 px-2 text-zinc-200 focus:outline-none",
                    placeholder: "Enter Password",
                    r#type: "password",
                    oninput: |ev| password_state.set(ev.value.clone())
                },
            },
            div {
                input {
                    class: "rounded-md bg-zinc-800 p-1 border-2 border-zinc-700 px-2 text-zinc-200 focus:outline-none",
                    placeholder: "Name",
                    value: "{name}",
                    oninput: |ev| name.set(ev.value.clone())
                },
            }
            div {
                input {
                    class: "rounded-md ml-20 bg-zinc-800 p-1 border-2 border-zinc-700 px-2 text-zinc-200 focus:outline-none",
                    placeholder: "Description",
                    value: "{description}",
                    oninput: |ev| description.set(ev.value.clone())
                },
            }
            div {
                input {
                    class: "rounded-md ml-40 bg-zinc-800 p-1 border-2 border-zinc-700 px-2 text-zinc-200 focus:outline-none",
                    placeholder: "Image",
                    value: "{image}",
                    oninput: |ev| image.set(ev.value.clone())
                },
            }
            div {
                input {
                    class: "rounded-md ml-64 bg-zinc-800 p-1 border-2 border-zinc-700 px-2 text-zinc-200 focus:outline-none",
                    placeholder: "Category",
                    value: "{category}",
                    oninput: |ev| category.set(ev.value.clone())
                },
            }
        }

        if !message.is_empty() {
            cx.render(rsx!{
                h3 {
                    class: "mt-3 mb-3 text-yellow-500 text-center",
                    "{message}"
                }
            })
        }
        div {
            class: "container mx-auto px-14 mt-5",
            div {
                class: "grid grid-cols-2 h-screen",
                div {
                    class: "border-r-2 border-zinc-400",
                    textarea {
                        id: "markdown-edit",
                        value: "{input_content}",
                        class: "h-full w-full bg-zinc-800 rounded-b-md rounded-l-md rounded-r-none p-3 text-zinc-300 focus:outline-none",
                        oninput: |ev| {
                            let raw_md = ev.value.clone();
                            input_content.set(raw_md.clone());
                            run_markdown_parse(cx, md_content.to_owned(), raw_md);
                        }
                    }
                },
                div {
                    class: "h-full w-full bg-zinc-800 rounded-r-md rounded-b-md",
                    div {
                        id: "post-content",
                        class: "p-3 text-zinc-300",
                        dangerous_inner_html: "{md_content}"
                    }
                }
            }
        }
    })
}
