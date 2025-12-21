use crate::components::page::Page;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div class="app">
            <Page/>
        </div>
    )
}
