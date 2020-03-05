use seed::{prelude::*, *};

struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self { val: 0 }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    vec![
        div![
            attrs!{
                At::Class => "container",
            },
            header![

                // logo
                a![ attrs!{At::Href => "/", At::Label => "Go back to homepage"},
                    div![
                        style![
                            St::Width => px(110),
                            St::Margin => "auto",
                        ],
                        svg![
                            // todo
                        ]
                    ]
                ],





                p![ attrs!{ At::Class => "bio" },
                    span![
                        "Personal website of Damian Szewczyk, a beginner rustacean with passion to learn about new technologies. "
                    ],
                    a![ attrs!{At::Href => "/about"}, "About me."]
                ],




                nav![
                    ul![
                        li![
                            a![
                                attrs!{
                                    At::Class => "icon",
                                    At::Href => "/",
                                    At::Title => "Go to homepage",
                                    //aria-label
                                },
                                svg![
                                    attrs!{
                                        At::ViewBox => "0 0 61 58"
                                    }
                                ]
                            ]
                        ],
                        li![],
                        li![],
                    ]
                ]
            ],

            main![
                    article![ attrs!{ At::Class => "homepage" },
                        p![ attrs!{ At::Class => "post-meta" },
                            "4th of February, 2020"
                        ],
                        a![ attrs!{ At::Href => "/link" },
                            h1![ "A Timelapse of Timelapse" ]
                        ],
                        "Timelapse is a little open-source screen recorder for macOS. It takes a screenshot every second and creates a
                        movie in the end. To celebrate its unlikely 1.0 release today, I present here a timelapse of this project's
                        journey. It just took ten years to get here. ",
                        a![ attrs!{ At::Href => "/link" }, "More »" ]
                    ]
            ],
        ],

        footer![
            div![
                attrs!{
                    At::Class => "body"
                },

                // for each footer div
                div![
                    h2![a![
                        attrs!{ At::Href => "/about" },
                        "About me"
                    ]],
                    p![
                        "Some more text about me..."
                    ],
                ],
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
