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
        let social_icon_style = "text-3xl w-9 text-center text-gray-500 hover:text-opacity-70 ";

        html! {
            <div class="flex flex-col w-4/5 p-0 my-8 bg-black">
            <h1 class="flex justify-center my-6 text-5xl">{"Damian Szewczyk"}</h1>
            <div class="flex justify-center">
                <a href="https://www.linkedin.com/in/szewczyk-damian" class={social_icon_style.to_owned()} + "fab fa-linkedin"></a>
                <a href="https://github.com/damszew" class={social_icon_style.to_owned()} + "fab fa-github"></a>
                <a href="https://gitlab.com/damszew" class={social_icon_style.to_owned()} + "fab fa-gitlab"></a>
            </div>
            <p class="m-6">
                <b>{"I'm a professional Rust developer"}</b> {" with some background in C++ and Yocto. Currently working on a more cloud/web related project."}
                <br/>
                <br/>
                {"In my free time I like tinkering 🔧, learning new technologies 💻 and gym 💪."}
            </p>
        </div>
        }
    }
}
