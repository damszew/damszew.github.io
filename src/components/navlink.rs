use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct Navlink {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub label: String,
    pub page: Route,
}

impl Component for Navlink {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        html! {
            <Anchor route={&self.props.page} >
                {&self.props.label}
            </Anchor>
        }
    }
}
