use crate::components::RawHtml;
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
                <img class="" src={self.image.clone()}/>
                <div class="post">
                    <h1>{&self.title}</h1>
                    <RawHtml inner_html={markdown_to_html(&self.description, &ComrakOptions::default())}/>
                    <a class="underline" href={self.link.clone()}>{&self.link_label}</a>
                </div>
            </article>
        }
    }
}
