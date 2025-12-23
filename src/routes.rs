use crate::components::content::readme::Readme;
use crate::components::page::Page;
use yew::prelude::*;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog/:id")]
    Blog { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Blog { id } => html! { <Page children={String::from(id)} /> },
        Route::Home => html! { <Page> <Readme /> </Page> },
        Route::NotFound => html! { <div><h1>{ "404 Not Found" }</h1></div> },
    }
}
