pub(crate) fn switch(routes: &Route) -> yew::Html {
    match routes {
        Route::Home => yew::html! { <h1>{ "Home" }</h1> },
        Route::Slide { id } => yew::html! {
            <slideshow::Present id={*id} />
        },
        Route::NotFound => yew::html! { <h1>{ "404" }</h1> },
    }
}

#[derive(Clone, yew_router::Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/slide/:id")]
    Slide { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}


use crate::slideshow;
