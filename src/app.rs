use crate::components::page::Page;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html!(
        <div class="app">
            <Page/>
        </div>
    )
}
