pub(crate) fn slide(title: &str, points: &[&str]) -> yew::Html {
    let bullets: Vec<_> = points
        .iter()
        .map(|point| yew::html! {<li> { point } </li>})
        .collect();

    yew::html! {
        <div class="section">
            <div class="content">
                <h1 class="title">{ title }</h1>
                <ul>{ bullets }</ul>
            </div>
        </div>
    }
}

pub(crate) fn with_image(title: &str, points: &[&str], image_src: impl ToString) -> yew::Html {
    let bullets: Vec<_> = points
        .iter()
        .map(|point| yew::html! {<li> { point } </li>})
        .collect();

    yew::html! {
        <div class="columns is-vcentered">
            <div class="column section is-half is-fullheight">
                <div class="content">
                    <h1 class="title">{ title }</h1>
                    <ul>{ bullets }</ul>
                </div>
            </div>
            <div class="column is-half is-fullheight">
                <div class="box">
                    <figure class="image is-square">
                        <img src={ image_src.to_string() } />
                    </figure>
                </div>
            </div>
        </div>
    }
}

pub(crate) fn with_icon(title: &str, points: &[&str], image_src: impl ToString) -> yew::Html {
    let bullets: Vec<_> = points
        .iter()
        .map(|point| yew::html! {<li> { point } </li>})
        .collect();

    yew::html! {
        <div class="columns is-vcentered">
            <div class="column section is-half is-fullheight">
                <div class="content">
                    <h1 class="title">{ title }</h1>
                    <ul>{ bullets }</ul>
                </div>
            </div>
            <div class="column is-half is-fullheight">
                <img src={ image_src.to_string() } />
            </div>
        </div>
    }
}
