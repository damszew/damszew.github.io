use crate::components::act_button::ActButton;
use crate::route::Route;
use yew::prelude::*;

pub struct Navbar {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active_page: Route,
}

pub enum Msg {
    ChangePage(Route),
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::ChangePage(page) => self.props.active_page = page,
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|route: Route| Msg::ChangePage(route));

        html! {
            <div id="sidebar" class="center-items">
                <ActButton label={"My projects"}, page={Route::ProjectsPage} is_active={if let Route::ProjectsPage = self.props.active_page {true} else {false} } on_page_change=callback.clone() />
                <ActButton label={"About Me"}, page={Route::AboutMePage} is_active={if let Route::AboutMePage = self.props.active_page {true} else {false} } on_page_change=callback />
            </div>
        }
    }
}
