use yew::prelude::*;

pub struct Navbar {
}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="sidebar" class="center-items">
                <a class="active">{"My projects"}</a>
                <a class="">{"About Me"}</a>
            </div>
        }
    }
}