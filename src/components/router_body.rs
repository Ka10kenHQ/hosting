use crate::routes::{Route, switch};
use yew::{Html, function_component, html};
use yew_router::{BrowserRouter, Switch};

#[function_component(RouterBody)]
pub fn router_body() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
