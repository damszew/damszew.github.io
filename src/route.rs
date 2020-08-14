use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/aboutme"]
    AboutMePage,
    #[to = "/"]
    ProjectsPage,
}
