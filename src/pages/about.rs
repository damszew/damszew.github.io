use yew::prelude::*;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="content-box about_me">
            <h1>{"Damian Szewczyk"}</h1>
            <div class="social_icons">
                <a href="https://www.linkedin.com/in/szewczyk-damian" class="fab fa-linkedin"></a>
                <a href="https://github.com/damszew" class="fab fa-github"></a>
                <a href="https://gitlab.com/damszew" class="fab fa-gitlab"></a>
            </div>
            <p>
                {"I'm professional C++ software developer also working with Yocto and CMake."}
                <br/>
                <br/>
                {"In my free time I like tinkering 🔧, learning new technologies 💻 and gym 💪.
                My recent new favourite is "}<em>{"Rust"}</em>
                {", so most of my personal projects are made with it."}
            </p>
        </div>
        }
    }
}
