#[yew::function_component(App)]
pub fn app() -> yew::Html {
    yew::html! {
        <yew_router::BrowserRouter>
            <yew_router::Switch<Route> render={yew_router::Switch::render(switch)} />
        </yew_router::BrowserRouter>
    }
}

const KEY_RIGHT: u32 = 39;
const KEY_LEFT: u32 = 37;

#[yew::function_component(Slideshow)]
fn slideshow_fn(props: &SlideProps) -> yew::Html {
    let history = yew_router::hooks::use_history().unwrap();

    let id = props.id;

    let onkeydown = yew::Callback::once(move |event: web_sys::KeyboardEvent| {
        let id = match event.key_code() {
            KEY_RIGHT => id.saturating_add(1),
            KEY_LEFT => id.saturating_sub(1),
            _ => id,
        };
        history.push(Route::Slide { id })
    });

    let thank_you = bullet_slide::slide("Thank you for listening", &["Goodbye"]);

    let focused_slide = all_slides::ALL_SLIDES.get(props.id).unwrap_or(&thank_you).clone();

    // Ugly hack to ensure the slide element is focused
    js! {
        setTimeout(
            () => {
                document.getElementById("slideshow").focus(); console.log("Focused slideshow")
            }
            , 200
        );
    }

    yew::html! {
        <div class="container" id="slideshow" tabindex="0" {onkeydown} autofocus=true>
        { focused_slide }
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

mod all_slides;
mod bullet_slide;
mod code_slide;
mod slideshow;
mod slido_slide;
mod this_slide;
mod title_slide;

use yew::html;
use yew_router::prelude::History;

use stdweb::js;
