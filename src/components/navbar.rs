use crate::components::act_button::ActButton;
use crate::route::Route;
use yew::prelude::*;

pub struct Navbar {}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
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
                <ActButton label={"My projects"}, page={Route::ProjectsPage} />
                <ActButton label={"About Me"}, page={Route::AboutMePage} />
            </div>
        }
    }
}
