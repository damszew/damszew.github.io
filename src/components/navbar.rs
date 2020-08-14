use crate::components::act_button::ActButton;
use crate::route::Route;
use yew::prelude::*;

pub struct Navbar {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active_page: Route,
}

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // TODO: Fix highlighting on currently active page
        html! {
            <div id="sidebar" class="center-items">
                <ActButton label={"My projects"}, page={Route::ProjectsPage} is_active={if let Route::ProjectsPage = self.props.active_page {true} else {false} } />
                <ActButton label={"About Me"}, page={Route::AboutMePage} is_active={if let Route::AboutMePage = self.props.active_page {true} else {false} } />
            </div>
        }
    }
}
