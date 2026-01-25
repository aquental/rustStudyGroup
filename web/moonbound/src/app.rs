use crate::component::blog_previews::BlogPreviews;
use crate::component::edit_post::EditPost;
use crate::component::view_post::ViewPost;

use component;
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn Navbar() -> impl IntoView {
    view![
    <h2>"Blog"</h2>
        <nav>
            <a href="/">"Home"</a>
            <a href="/edit">"Edit Blog"</a>
        </nav>
    ]
}

#[component]
pub fn Statusbar() -> impl IntoView {
    view![
        <footer>
            <p>"© 2026 Moonbound. All rights reserved."</p>
        </footer>
    ]
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/moonbound.css"/>

        // sets the document title
        <Title text="Welcome to Moonbound"/>

        <Navbar/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=component::blog_previews::BlogPreviews/>
                    <Route path="/edit/:post_id?" view=component::edit_post::EditPost/>
                    <Route path="/view/:post_id?" view=component::view_post::ViewPost/>
                </Routes>
            </main>
        </Router>


        <Statusbar/>
    }
}
