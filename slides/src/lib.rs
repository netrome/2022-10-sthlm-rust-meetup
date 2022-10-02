#[yew::function_component(App)]
pub fn app() -> yew::Html {
    yew::html! {
        <yew_router::BrowserRouter>
            <yew_router::Switch<route::Route> render={yew_router::Switch::render(route::switch)} />
        </yew_router::BrowserRouter>
    }
}

#[derive(yew::Properties, PartialEq)]
struct SlideProps {
    pub id: usize,
}

// Just module declarations from here

mod bullet_slide;
mod code_slide;
mod content;
mod route;
mod slideshow;
mod slido_slide;
mod this_slide;
mod title_slide;
