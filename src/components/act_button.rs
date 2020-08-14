use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct ActButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub label: String,
    pub page: Route,
    pub is_active: bool,
    pub on_page_change: Callback<Route>,
}

pub enum Msg {
    ChangePage,
}

impl Component for ActButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangePage => self.props.on_page_change.emit(self.props.page.clone()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ChangePage);
        let active_class = if self.props.is_active { "active" } else { "" };

        type Anchor = RouterAnchor<Route>;
        html! {
            <Anchor route={&self.props.page}>
                <a class=active_class onclick=onclick>{&self.props.label}</a>
            </Anchor>
        }
    }
}
