use crate::api;
use crate::types::Post;
use anyhow::Error;
use yew::{format::Json, prelude::*, services::fetch::FetchTask};

pub enum Msg {
    GetPosts,
    GetPostsSuccess(Vec<Post>),
    GetPostsError(Error),
}

struct State {
    posts: Vec<Post>,
    get_posts_error: Option<Error>,
    get_posts_loaded: bool,
}

pub struct Projects {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPosts);

        Self {
            state: State {
                posts: Vec::new(),
                get_posts_error: None,
                get_posts_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetPosts => {
                self.state.get_posts_loaded = false;
                let handler = self
                    .link
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
                self.state.posts = posts;
                self.state.get_posts_loaded = true;
                true
            }
            Msg::GetPostsError(error) => {
                self.state.get_posts_error = Some(error);
                self.state.get_posts_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let posts = self
            .state
            .posts
            .iter()
            .map(|post| post.to_html())
            .collect::<Vec<_>>();

        if !self.state.get_posts_loaded {
            html! {
              <div class="center-box center-items">
                <div class="lds-dual-ring"></div>
              </div>
            }
        } else if self.state.get_posts_error.is_some() {
            html! {
              <div class="center-box center-items">
                <div class="black-box">
                    <i class="fas fa-times"></i>
                    <span class="error-msg">{"Error loading posts! :("}</span>
                </div>
              </div>
            }
        } else {
            html! { <>{posts}</> }
        }
    }
}
