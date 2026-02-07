use crate::components::{router_body::RouterBody, sidebar::Sidebar};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div class="app">
            <Sidebar/>
            <main>
                <RouterBody />
            </main>
        </div>
    )
}
