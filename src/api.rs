use crate::types::Post;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_posts(callback: FetchCallback<Vec<Post>>) -> FetchTask {
    let req = Request::get("/static/posts.json").body(Nothing).unwrap();

    FetchService::fetch(req, callback).unwrap()
}
