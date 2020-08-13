use yew::prelude::*;

mod post {

    pub const POSTS: &[Post] = &[
        Post{
            title: "First open source contribution",
            image: "static/merged_pr.png",
            description: "Recently I started messing around with [godot-rust](https://github.com/godot-rust/godot-rust)
                and while doing so, I ran into one of these problems that felt like they should be easy to solve, but I couldn't get it to work.
                After some time, I decided to ask [\"How to use Godot's resources in rust bindings?\"](https://github.com/godot-rust/godot-rust/issues/559)
                Perhaps [\"Getting Started book\"](https://godot-rust.github.io/book/getting-started.html) will have a chapter on this topic,
                but for now, the simple example which I have prepared seems to be \"good enough\" solution to show others the basics of this library.


    It was my first ever open source contribution. This is just a small example
                but first **merge** to open source project feels amazing.
                I can't wait to contribute to OSS again 🔥.
                ",
            link_label: "Link to PR",
            link: "https://github.com/godot-rust/godot-rust/pull/563",
        },
        Post{
            title: "Personal web page",
            image: "static/personal_page_screenshot.png",
            description: "My very first pet project that I want to show to the world.
                Being more of a system level dev, I have always wanted to create something more visual.
                Tackling unknown domain with newly learned language was quite a challenge.
                But thanks to [rust lang](https://www.rust-lang.org/) community, beginner friendly examples provided by [seed-rs](https://seed-rs.org/)
                and incomparably good explanation of fundamentals from [w3schools](https://www.w3schools.com/), it was nothing less than a pleasant challenge.",
            link_label: "Check it out at gitlab",
            link: "https://gitlab.com/damszew/damszew.gitlab.io",
        },
    ];

    type Href = str;

    pub struct Post<'a> {
        pub title: &'a str,
        pub image: &'a Href,
        pub description: &'a str,
        pub link_label: &'a str,
        pub link: &'a Href,
    }
}

use post::Post;

pub struct Projects {
    posts: &'static [Post<'static>],
}

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            posts: &post::POSTS,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let posts: Vec<Html> = self
            .posts
            .iter()
            .map(|post: &post::Post| {
                html! {
                    <article class="content-box">
                        <img class="" src={&post.image}/>
                        <div class="">
                            <h1>{&post.title}</h1>
                            <p>{&post.description}</p> // TODO: this parse markdown to html
                            <a href={post.link}>{&post.link_label}</a>
                        </div>
                    </article>
                }
            })
            .collect();

        html! { <>{posts}</> }
    }
}
