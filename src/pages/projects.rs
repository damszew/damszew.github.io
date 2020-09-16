use crate::api;
use crate::types::Post;
use anyhow::Error;
use yew::{prelude::*, services::fetch::FetchTask, format::Json};

pub enum Msg {
    GetPosts,
    GetPostsSuccess(Vec<Post>),
    GetPostsError(Error),
}

pub struct Projects {
    posts: Vec<Post>,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPosts);

        Self {
            posts: Vec::new(),
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetPosts => {
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Post>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(products) => Msg::GetPostsSuccess(products),
                                Err(err) => Msg::GetPostsError(err),
                            }
                        });
                self.task = Some(api::get_posts(handler));
                true
            }
            Msg::GetPostsSuccess(posts) => {
                self.posts = posts;
                true
            }
            Msg::GetPostsError(error) => {
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let posts = self
            .posts
            .iter()
            .map(|post| post.to_html())
            .collect::<Vec<_>>();

        html! { <>{posts}</> }
    }
}
