use crate::components::RawHTML;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use yew::{html, Html};

type Href = String;

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub title: String,
    pub image: Href,
    pub description: String,
    pub link_label: String,
    pub link: Href,
}

impl Post {
    pub fn to_html(&self) -> Html {
        html! {
            <article class="content-box">
                <img class="" src={&self.image}/>
                <div class="post">
                    <h1>{&self.title}</h1>
                    <RawHTML inner_html={markdown_to_html(&self.description, &ComrakOptions::default())}/>
                    <a href={self.link.clone()}>{&self.link_label}</a>
                </div>
            </article>
        }
    }
}
