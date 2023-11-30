use rand::seq::SliceRandom;

use dioxus::prelude::*;

pub fn JourneyPage(cx: Scope) -> Element {
    cx.render(rsx!{
        section { class: "bg-transparent",
            div { class: "gap-8 items-center py-8 px-4 mx-auto max-w-screen-xl xl:gap-16 md:grid md:grid-cols-2 sm:py-16 lg:px-6",
                img { class: "w-3/4 rounded-full",
                    alt: "me",
                    src: "/images/dallegenerated.png",
                }
                div { class: "mt-4 md:mt-0",
                    h2 { class: "mb-4 text-4xl tracking-tight font-extrabold text-white",
                        "A bit more about me"
                    }
                    p { class: "font-light text-gray-500 md:text-lg dark:text-gray-400",
                        "My Computer Science journey started when I was 7 years old, a strange age I'd agree. It began with a new found love for Game Development. A couple school friends on the "
                        a {
                            href: "https://roblox.com",
                            class: "text-sky-400 hover:animate-pulse",
                            "Roblox"
                        },
                        " platform had introduced me to a popular Pokémon game at the time. Funnily enough my first thought was, 'wow, this is so cool, I wanna make a game!'. The rest is history."
                    }
                    // a { class: "inline-flex items-center text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:focus:ring-primary-900",
                    //     href: "#",
                    //     "Get started"
                    //     svg { class: "ml-2 -mr-1 w-5 h-5",
                    //         view_box: "0 0 20 20",
                    //         xmlns: "http://www.w3.org/2000/svg",
                    //         fill: "currentColor",
                    //         path { 
                    //             fill_rule: "evenodd",
                    //             clip_rule: "evenodd",
                    //             d: "M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z",
                    //         }
                    //     }
                    // }
                }
            }
            // div {
            //     class: "flex-center",
            //     img {
            //         src: "/images/down-arrow.svg",
            //         onclick: |evt| scroll_down(evt),
            //         class: "w-10 text-sky-400 animate-bounce"
            //     }
            // }
            
        }

        Timeline {}
       
    })
}

pub fn Timeline(cx: Scope) -> Element {
    cx.render(rsx!{
        section { class: "bg-transparent",
    div { class: "container max-w-5xl px-4 py-12 mx-auto",
        div { class: "grid gap-4 mx-4 sm:grid-cols-12",
            div { class: "col-span-12 sm:col-span-3",
                div { class: "text-center sm:text-left mb-14 before:block before:w-24 before:h-3 before:mb-5 before:rounded-md before:mx-auto sm:before:mx-0 before:dark:bg-sky-400",
                    h3 { class: "text-3xl font-semibold text-white",
                        "My Journey"
                    }
                    span { class: "text-sm font-semibold text-gray-400",
                        "I've had a heck of a ride!"
                    }
                }
            }
            div { class: "relative col-span-12 px-4 space-y-6 sm:col-span-9",
                div { class: "col-span-12 space-y-12 relative px-4 sm:col-span-8 sm:space-y-8 sm:before:absolute sm:before:top-2 sm:before:bottom-0 sm:before:w-0.5 sm:before:-left-3 before:dark:bg-gray-700",
                    TimelineEvent {
                        title: "Started Coding",
                        description: "Learnt basic game development with the Roblox Studio Engine and Lua. Created a few basic sword fighting games. Fiddled with Scratch a little during this period too.",
                        time: "2015"
                    }
                    TimelineEvent {
                        title: "Created my first website",
                        description: "Learnt HTML and basic CSS from a book I borrowed from my local library",
                        time: "2015-2016"
                    }
                    TimelineEvent {
                        title: "Text adventures with Python",
                        description: "Created silly text adventures with Python with the help of my father. Remember when input used to be called raw_input?"
                        time: "2016-2017"
                    }
                    TimelineEvent {
                        title: "Discord Bot Development",
                        description: "Seriously levelled up my Python skills by exploring discord bot development. I learnt so many things. Networking, Databases, Async Programming and more!",
                        time: "2018-2019"
                    }
                    TimelineEvent {
                        title: "Game Development",
                        description: "Got back into Game Development with Unity and C# for a little while.",
                        time: "2019-2020"
                    }
                    TimelineEvent {
                        title: "Began to explore more topics such as Full Stack Web Development",
                        description: "Created a few sites using Django surrounding the topic of anime, and founded a successful anime community.",
                        time: "2020-2021"
                    }
                    TimelineEvent {
                        title: "Slowly began to explore Data Science and Machine Learning",
                        description: "I dropped in and out of exploring data science and machine learning while still working on a few large discord bots.",
                        time: "2021-2022"
                    }
                    TimelineEvent {
                        title: "PERSE Team Coding Challenge",
                        description: "Participated with a friend who was new to programming at the time, and we gained a distinction and best in school!",
                        time: "2022"
                    }
                    TimelineEvent {
                        title: "Machine Learning & Data Science",
                        description: "Decided to invest my time into solely Machine Learning and Data Science in an attempt to create something meaningful.",
                        time: "2022-Present"
                    }
                }
            }
        }
    }
}

    })
}

#[inline_props]
pub fn TimelineEvent<'a>(
    cx: Scope,
    title: &'a str,
    description: &'a str,
    time: &'a str,
) -> Element {
    let colors = vec!["rose", "emerald", "sky"];
    let color = colors.choose(&mut rand::thread_rng()).unwrap();
    cx.render(rsx!{
        div { class: "flex flex-col sm:relative sm:before:absolute sm:before:top-2 sm:before:w-4 sm:before:h-4 sm:before:rounded-full sm:before:left-[-35px] sm:before:z-[1] before:bg-{color}-400",
                        h3 { class: "text-xl font-semibold tracking-wide text-white",
                            "{title}"
                        }
                        time { class: "text-xs tracking-wide uppercase text-gray-400",
                            "{time}"
                        }
                        p { class: "mt-3 text-gray-300",
                            "{description}"
                        }
                    }
    })
}
