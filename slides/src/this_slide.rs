pub(crate) fn slide() -> yew::Html {

    let source_code = include_str!("this_slide.rs");

    yew::html! {
        <div class="section">
            <h1 class="title">{ "Can you guess what this code does?" }</h1>
            <pre>{ source_code }</pre>
        </div>
    }
}
