fn main() {
    yew::start_app::<App>();
}

#[yew::function_component(App)]
fn app() -> yew::Html {
    yew::html! {
        <yew_router::BrowserRouter>
            <yew_router::Switch<Route> render={yew_router::Switch::render(switch)} />
        </yew_router::BrowserRouter>
    }
}

const KEY_RIGHT: u32 = 39;
const KEY_LEFT: u32 = 37;

#[yew::function_component(Slide)]
fn slide(props: &SlideProps) -> yew::Html {
    let msg = format!("Hello slide {}", props.id);

    let history = yew_router::hooks::use_history().unwrap();

    let id = props.id;

    let onkeydown = yew::Callback::once(move |event: web_sys::KeyboardEvent| {
        let id = match event.key_code() {
            KEY_RIGHT => id + 1,
            KEY_LEFT => id - 1,
            _ => 1337,
        };
        history.push(Route::Slide { id })
    });

    yew::html! {
        <div tabindex="0" {onkeydown}>
            <h1>{ msg }</h1>
        </div>
    }
}

#[derive(yew::Properties, PartialEq)]
pub struct SlideProps {
    pub id: usize,
}

#[derive(Clone, yew_router::Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/slide/:id")]
    Slide { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> yew::Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Slide { id } => html! {
            <Slide id={*id} />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

use yew::html;
use yew_router::prelude::History;
