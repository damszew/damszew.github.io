use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct Navlink {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Clicked,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub label: String,
    pub page: Route,
    pub active: bool,
    pub active_class: String,

    #[prop_or(Callback::noop())]
    pub on_clicked: Callback<Msg>,
}

impl Component for Navlink {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => self.props.on_clicked.emit(msg),
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        let classes = if self.props.active {
            self.props.active_class.clone()
        } else {
            "".into()
        };

        let on_click = self.link.callback(|_| Msg::Clicked);
        let page = self.props.page.clone();

        html! {
            <Anchor route={page} classes={classes}>
                <div onclick={on_click}>
                    {&self.props.label}
                </div>
            </Anchor>
        }
    }
}
