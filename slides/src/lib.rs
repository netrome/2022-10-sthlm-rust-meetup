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
fn slideshow(props: &SlideProps) -> yew::Html {
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

    let all_slides = vec![
        menti_slide(),
        title_slide(HeroStyle::Primary, "Goodbye JS 👋 Hello Rust", "Exploring front-end development with Yew.rs"),
        bullet_slide("Agenda", &["Intro", "Are we frontend yet?", "Enter yew.rs", "Getting practical", "Summary"]),
        bullet_slide("About me", &["Algorithm Engineer @ Tobii", "AI Developer @ Norna", "Software Engineer @ Klarna", "Engineering Manager @ Validio"]),
        bullet_slide(
            "Some of my web-heavy projects",
            &["Vacation planner (Stealth startup)"
             ,"Travel blog (Personal)"
             ,"Data cleaning tool (Tobii)"
             ,"Data annotation tool (Norna)"
             ,"Wedding website (Personal)"
             ]
        ),
        bullet_slide(
            "Which stack would I choose today?",
            &["Extensive web UI, large team -> JS/TS front-end, Rust backend"
             ,"Content-heavy website -> No client side logic, Rust backend"
             ,"MVP/smaller app, small team -> fullstack JS or fullstack Rust"
            ]
        ),
        bullet_slide(
            "Benefits of using a single language",
            &["Code reuse"
             ,"Cognitive load"
             ,"Quick onboarding"
            ]
        ),
        title_slide(HeroStyle::Info, "Are we frontend yet?", ""),
        bullet_slide(
            "Are we frontend yet?",
            &["Can we run Rust in the browser?"
             ,"Can we interface with all web APIs?"
             ,"Can we interoperate with existing JS libraries?"
             ,"Are there any good frameworks out there already?"
            ]
        ),
        bullet_slide_with_icon(
            "WebAssembly",
            &["Portable & compact execution format"
             ,"Supported by all major browsers"
             ,"Integrates well with JavaScript"
             ,"https://rustwasm.github.io/docs"
            ],
            "http://localhost:9090/wasm-ferris.png"
        ),
        bullet_slide_with_image(
            "Rust front-end frameworks",
            &["Seed", "Yew", "Sauron", "Syncamore", "Percy", "Iced", "Mogwai", "Dioxus"],
            "http://localhost:9090/rust_frontend.png"
        ),
        title_slide(HeroStyle::Info, "Enter yew.rs", ""),
        bullet_slide_with_image(
            "Yew",
            &["Component based framework"
             ,"Virtual DOM"
             ,"Very similar to React"
            ],
            "http://localhost:9090/yew_deps.png"
        ),
        bullet_slide_with_image(
            "Why Yew?",
            &["Most popular"
             ,"Familiar interface"
             ,"Hackable"
             ,"Easy to get started"
            ],
            "http://localhost:9090/yew_compiler_error.png"
        ),
        bullet_slide_with_image(
            "Yew stats",
            &["2017 first commit"
             ,"6550 dependents on GitHub"
             ,"333 contributors"
             ,"388 925 downloads from crates.io"
            ],
            "http://localhost:9090/yew_contributors.png"
        ),
        bullet_slide_with_image(
            "A minimal yew app",
            &["10 loc main.rs"
             ,"5 loc index.html"
             ,"1 dependency in cargo.toml"
            ],
            "http://localhost:9090/minimal_app_tree.png"
        ),
        code_slide::slide!("../../minimal-app/Cargo.toml"),
        code_slide::slide!("../../minimal-app/src/main.rs"),
        code_slide::slide!("../../minimal-app/index.html"),
        title_slide(HeroStyle::Info, "Yew in practice", ""),
        this_slide::slide(),
        code_slide::slide!("code_slide.rs"),
    ];

    let thank_you = bullet_slide("Thank you for listening", &["Goodbye"]);

    let focused_slide = all_slides.get(props.id).unwrap_or(&thank_you).clone();

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

fn image_slide(src: impl ToString) -> yew::Html {
    yew::html! {
        <figure class="image is-fullheight">
            <img src={ src.to_string() } />
        </figure>
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

fn bullet_slide(title: &str, points: &[&str]) -> yew::Html {
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

fn bullet_slide_with_image(title: &str, points: &[&str], image_src: impl ToString) -> yew::Html {
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

fn bullet_slide_with_icon(title: &str, points: &[&str], image_src: impl ToString) -> yew::Html {
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

fn menti_slide() -> yew::Html {
    yew::html! {
        <div style="position: relative; padding-bottom: 56.25%; padding-top: 35px; height: 0; overflow: hidden;"><iframe sandbox="allow-scripts allow-same-origin allow-presentation" allowfullscreen=true allowtransparency="true" frameborder="0" height="315" src="https://wall.sli.do/event/uD6zQCLTtdC4Zzmf2kn48y?section=db5bc5c5-b659-4e4b-a9e1-fd8a23f7a687" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;" width="420"></iframe></div>
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

mod code_slide;
mod this_slide;

use yew::html;
use yew_router::prelude::History;

use stdweb::js;
