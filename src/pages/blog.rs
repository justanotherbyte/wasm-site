use dioxus::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use dioxus_markdown::Markdown;

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
            image: "/images/loading.gif".into(),
            category: "Loading...".into()
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
