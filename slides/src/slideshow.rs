#[yew::function_component(Present)]
pub(crate) fn present(props: &SlideProps) -> yew::Html {
    let thank_you = title_slide::primary("Thank you for listening 👋", "");
    let focused_slide = content::SLIDES.get(props.id).unwrap_or(&thank_you).clone();

    let onkeydown = slide_navigation_callback(props.id);

    js::focus_slideshow(); // Ensure #slideshow is focused to capture key events
                           //
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

fn slide_navigation_callback(id: usize) -> yew::Callback<web_sys::KeyboardEvent> {
    let history = yew_router::hooks::use_history().unwrap();

    yew::Callback::once(move |event: web_sys::KeyboardEvent| {
        let id = match event.key_code() {
            KEY_RIGHT => id.saturating_add(1),
            KEY_LEFT => id.saturating_sub(1),
            _ => id,
        };
        history.push(route::Route::Slide { id })
    })
}

const KEY_RIGHT: u32 = 39;
const KEY_LEFT: u32 = 37;

use crate::content;
use crate::js;
use crate::route;
use crate::title_slide;

use yew_router::prelude::History;
