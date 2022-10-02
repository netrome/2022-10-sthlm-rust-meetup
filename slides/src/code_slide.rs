#[macro_export]
macro_rules! slide {
    ($file:expr) => {
        yew::html! {
            <div class="section">
                <h1 class="title">{ $file }</h1>
                <pre>{ include_str!($file) }</pre>
            </div>
        }
    };
}

pub(crate) use slide;
