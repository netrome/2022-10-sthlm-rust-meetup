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

    let focused_slide = content::SLIDES.get(props.id).unwrap_or(&thank_you).clone();

    // Ensure #slideshow is focused to capture key events
    js::focus_slideshow();

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

use crate::bullet_slide;
use crate::content;
use crate::js;
use crate::route;

use yew_router::prelude::History;
