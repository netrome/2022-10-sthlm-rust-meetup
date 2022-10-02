pub(crate) fn primary(title: &str, subtitle: &str) -> yew::Html {
    title_slide(HeroStyle::Primary, title, subtitle)
}

pub(crate) fn info(title: &str, subtitle: &str) -> yew::Html {
    title_slide(HeroStyle::Info, title, subtitle)
}

fn title_slide(hero_style: HeroStyle, title: &str, subtitle: &str) -> yew::Html {
    let section_class = format!("hero is-fullheight {}", hero_style.to_string());

    yew::html! {
        <section class={section_class}>
            <div class="hero-body">
                <div class="">
                    <h1 class="title"> { title } </h1>
                    <h2 class="subtitle"> { subtitle } </h2>
                </div>
            </div>
        </section>
    }
}

enum HeroStyle {
    Primary,
    Info,
}

impl ToString for HeroStyle {
    fn to_string(&self) -> String {
        match self {
            Self::Primary => "is-primary".to_owned(),
            Self::Info => "is-info".to_owned(),
        }
    }
}
