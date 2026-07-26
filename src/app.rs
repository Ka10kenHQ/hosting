use crate::{components::router_body::RouterBody, routes::Route};
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <header class="site-header" id="site-header">

                <Link<Route> classes="site-brand" to={Route::Home}>
                    <span class="site-brand__mark">{"MK"}</span>
                    <span>{"Mate Kopaliani"}</span>
                </Link<Route>>

                <nav class="site-nav" aria-label="Primary navigation">
                    <Link<Route> to={Route::Home}>{"About"}</Link<Route>>
                    <Link<Route> to={Route::Project { name: "vortexnote".to_string() }}>{"Projects"}</Link<Route>>
                    <a href="https://github.com/Ka10ken1" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                </nav>

            </header>

            <main>
                <RouterBody />
            </main>

        </div>
    }
}
