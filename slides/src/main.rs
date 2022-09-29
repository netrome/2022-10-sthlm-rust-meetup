#[yew::function_component(App)]
fn app() -> yew::Html {
    yew::html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
