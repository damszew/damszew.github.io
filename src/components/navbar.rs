use crate::components::navlink::Navlink;
use crate::route::Route;
use yew::prelude::*;

pub struct Navbar {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active_page: u32,
}

pub enum Msg {
    ChangeItem(u32),
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeItem(id) => self.props.active_page = id,
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
                    label={"My projects"},
                    page={Route::ProjectsPage},
                    active={&self.props.active_page == &0},
                    active_class={"active"},
                    on_clicked={self.link.callback(|_|{ Msg::ChangeItem(0) })}
                />
                <Navlink
                    label={"About Me"},
                    page={Route::AboutMePage},
                    active={&self.props.active_page == &1},
                    active_class={"active"},
                    on_clicked={self.link.callback(|_|{ Msg::ChangeItem(1) })}
                />
            </div>
        }
    }
}
