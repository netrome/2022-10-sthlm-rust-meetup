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

#[yew::function_component(Slide)]
fn slide(props: &SlideProps) -> yew::Html {
    let msg = format!("Hello slide {}", props.id);

    yew::html! {
        <h1>{ msg }</h1>
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
    Slide{id: usize},
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> yew::Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Slide{id} => html! {
            <Slide id={*id} />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

use yew::html;
