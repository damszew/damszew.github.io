use crate::components::navlink::Navlink;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::RouteService;

pub struct Navbar {
    link: ComponentLink<Self>,
    current_page: yew_router::route::Route<()>,
    _route_service: RouteService,
}

#[derive(Properties, Clone)]
pub struct Props {}

pub enum Msg {
    RouteChanged(yew_router::route::Route<()>),
    Change(Route),
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service = RouteService::new();
        let current_page = route_service.get_route();

        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);

        Self {
            link,
            current_page,
            _route_service: route_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.current_page = route,
            Msg::Change(page) => self.current_page = page.into(),
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="sidebar" class="center-items">
                <Navlink
                    label={"My projects"}
                    page={Route::ProjectsPage}
                    active={self.current_page == Route::ProjectsPage.into()}
                    active_class={"active"}
                    on_clicked={self.link.callback(|_| Msg::Change(Route::ProjectsPage))}
                />
                <Navlink
                    label={"About Me"}
                    page={Route::AboutMePage}
                    active={self.current_page == Route::AboutMePage.into()}
                    active_class={"active"}
                    on_clicked={self.link.callback(|_| Msg::Change(Route::AboutMePage))}
                />
            </div>
        }
    }
}
