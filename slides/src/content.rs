pub(crate) const SLIDES: Lazy<Vec<yew::Html>> = Lazy::new(|| {
    vec![
        slido_slide::slide(),
        title_slide::primary(
            "Goodbye JS 👋 Hello Rust",
            "Exploring front-end development with Yew.rs",
        ),
        bullet_slide::slide(
            "Agenda",
            &[
                "Intro",
                "Are we frontend yet?",
                "Enter yew.rs",
                "Getting practical",
                "Conclusion",
            ],
        ),
        bullet_slide::slide(
            "About me",
            &[
                "Algorithm Engineer @ Tobii",
                "AI Developer @ Norna",
                "Software Engineer @ Klarna",
                "Engineering Manager @ Validio",
            ],
        ),
        bullet_slide::with_image(
            "Some of my web-heavy projects",
            &[
                "Vacation planner (Stealth startup)",
                "Travel blog (Personal)",
                "Data cleaning tool (Tobii)",
                "Data annotation tool (Norna)",
                "Klarna Online (Klarna)",
                "Wedding website (Personal)",
            ],
            "http://localhost:9090/web_frameworks.png",
        ),
        bullet_slide::with_image(
            "Which stack would I choose today?",
            &[
                "Extensive web UI, large team -> JS/TS front-end, Rust backend",
                "Content-heavy website -> No client side logic, Rust backend",
                "MVP/smaller app, small team -> fullstack JS or fullstack Rust",
            ],
            "http://localhost:9090/full_stack.jpg",
        ),
        bullet_slide::slide(
            "Benefits of using a single language",
            &["Code reuse", "Cognitive load", "Quick onboarding"],
        ),
        title_slide::info("Are we frontend yet?", ""),
        bullet_slide::slide(
            "Are we frontend yet?",
            &[
                "Can we run Rust in the browser?",
                "Can we interface with all web APIs?",
                "Can we interoperate with existing JS libraries?",
                "Are there any good frameworks out there already?",
            ],
        ),
        bullet_slide::with_icon(
            "WebAssembly",
            &[
                "Portable & compact execution format",
                "Supported by all major browsers",
                "Integrates well with JavaScript",
                "https://rustwasm.github.io/docs",
            ],
            "http://localhost:9090/wasm-ferris.png",
        ),
        bullet_slide::with_image(
            "Rust front-end frameworks",
            &[
                "Seed",
                "Yew",
                "Sauron",
                "Syncamore",
                "Percy",
                "Iced",
                "Mogwai",
                "Dioxus",
            ],
            "http://localhost:9090/rust_frontend.png",
        ),
        title_slide::info("Enter yew.rs", ""),
        bullet_slide::with_image(
            "Yew",
            &[
                "Component based framework",
                "Virtual DOM",
                "Very similar to React",
            ],
            "http://localhost:9090/yew_deps.png",
        ),
        bullet_slide::with_image(
            "Why Yew?",
            &[
                "Well maintained",
                "Easy to get started",
                "Intuitive",
                "Hackable",
            ],
            "http://localhost:9090/yew_compiler_error.png",
        ),
        bullet_slide::with_image(
            "Yew stats",
            &[
                "2017 first commit",
                "6550 dependents on GitHub",
                "333 contributors",
                "388 925 downloads from crates.io",
            ],
            "http://localhost:9090/yew_contributors.png",
        ),
        bullet_slide::with_image(
            "A minimal yew app",
            &[
                "10 loc main.rs",
                "5 loc index.html",
                "1 dependency in cargo.toml",
            ],
            "http://localhost:9090/minimal_app_tree.png",
        ),
        code_slide::slide!("../../minimal-app/Cargo.toml"),
        code_slide::slide!("../../minimal-app/src/main.rs"),
        code_slide::slide!("../../minimal-app/index.html"),
        title_slide::info("Yew in practice", ""),
        this_slide::slide(),
        bullet_slide::with_image(
            "This presentation",
            &[
                "Written in Rust using Yew",
                "Served with `trunk serve`",
                "Content is a Vec<yew::Html>",
                "Styled with Bulma CSS",
            ],
            "http://localhost:9090/slide_architecture.png",
        ),
        code_slide::slide!("code_slide.rs"),
        code_slide::slide!("main.rs"),
        code_slide::slide!("lib.rs"),
        code_slide::slide!("route.rs"),
        code_slide::slide!("slideshow.rs"),
        code_slide::slide!("js.rs"),
        code_slide::slide!("bullet_slide.rs"),
        slido_slide::slide(),
        code_slide::slide!("content.rs"),
        bullet_slide::slide(
            "Are we frontend yet?",
            &[
                "Can we run Rust in the browser?",
                "Can we interface with all web APIs?",
                "Can we interoperate with existing JS libraries?",
                "Are there any good frameworks out there already?",
            ],
        ),
        bullet_slide::with_image(
            "Are we frontend yet? - Yes",
            &[
                "Can we run Rust in the browser? - Yes",
                "Can we interface with all web APIs? - Yes",
                "Can we interoperate with existing JS libraries? - Yes",
                "Are there any good frameworks out there already? - Yes",
            ],
            "http://localhost:9090/oprah_yes.jpg",
        ),
    ]
});

pub(crate) const THANK_YOU: Lazy<yew::Html> =
    Lazy::new(|| title_slide::primary("Thank you for listening 👋", ""));

use crate::bullet_slide;
use crate::code_slide;
use crate::slido_slide;
use crate::this_slide;
use crate::title_slide;

use once_cell::sync::Lazy;
