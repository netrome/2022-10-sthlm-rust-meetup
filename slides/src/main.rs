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

#[yew::function_component(Slideshow)]
fn slideshow(props: &SlideProps) -> yew::Html {
    let history = yew_router::hooks::use_history().unwrap();

    let id = props.id;

    let onkeydown = yew::Callback::once(move |event: web_sys::KeyboardEvent| {
        let id = match event.key_code() {
            KEY_RIGHT => id.checked_add(1).unwrap_or(usize::MAX),
            KEY_LEFT => id.checked_sub(1).unwrap_or(0),
            _ => 1337,
        };
        history.push(Route::Slide { id })
    });

    let all_slides = vec![
        bullet_slide(
            "Never gonna",
            &["Give you up", "Let you down", "Tell a lie", "Hurt you"],
        ),
        bullet_slide("I am", &["Mother of bugs", "Typer of Keyboard"]),
    ];

    let default_slide = bullet_slide("No more content", &["You went too far", "Go back!"]);

    let focused_slide = all_slides.get(props.id).unwrap_or(&default_slide).clone();

    yew::html! {
        <div id="slideshow" tabindex="0" {onkeydown} autofocus=true>
        { focused_slide }
        </div>
    }
}

fn bullet_slide(title: &str, points: &[&str]) -> yew::Html {
    let bullets: Vec<_> = points
        .iter()
        .map(|point| yew::html! {<li> { point } </li>})
        .collect();

    yew::html! {
        <div>
            <h1>{ title }</h1>
            <ul>{ bullets }</ul>
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
            <Slideshow id={*id} />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

use yew::html;
use yew_router::prelude::History;
