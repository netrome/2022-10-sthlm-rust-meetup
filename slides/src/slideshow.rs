#[yew::function_component(Present)]
pub(crate) fn present(props: &SlideProps) -> yew::Html {
    let history = yew_router::hooks::use_history().unwrap();

    let id = props.id;

    let onkeydown = yew::Callback::once(move |event: web_sys::KeyboardEvent| {
        let id = match event.key_code() {
            KEY_RIGHT => id.saturating_add(1),
            KEY_LEFT => id.saturating_sub(1),
            _ => id,
        };
        history.push(route::Route::Slide { id })
    });

    let thank_you = bullet_slide::slide("Thank you for listening", &["Goodbye"]);

    let focused_slide = all_slides::ALL_SLIDES
        .get(props.id)
        .unwrap_or(&thank_you)
        .clone();

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
pub(crate) struct SlideProps {
    pub id: usize,
}

const KEY_RIGHT: u32 = 39;
const KEY_LEFT: u32 = 37;

use crate::all_slides;
use crate::bullet_slide;
use crate::route;

use stdweb::js;
use yew_router::prelude::History;
