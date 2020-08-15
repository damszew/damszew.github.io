use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Navbar;
use crate::pages::About;
use crate::pages::Projects;
use crate::route::Route;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::ProjectsPage => html! {<Projects/>},
            Route::AboutMePage => html! {<About/>},
        });

        html! {
            <div>
                // TODO: Fix Navlink highlight when starting from different route
                <Navbar active_page={0} />
                <div id="content">
                    <Router<Route> render=render/>
                </div>
            </div>
        }
    }
}
