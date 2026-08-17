use crate::components::{
    content::{
        projects::{
            floating_point_unit::FloatingPointUnit, llm_debate::LlmDebate,
            only_vim::OnlyVim, ragtrace::Ragtrace, vortexnote::Vortexnote,
            watchclean::Watchclean,
        },
        readme::Readme,
        thesis::Thesis,
    },
    page::Page,
};
use yew::prelude::*;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route
{
    #[at("/")]
    HomeBase,
    #[at("/hosting/")]
    Home,
    #[at("/hosting/about")]
    Readme,
    #[at("/hosting/blog/:id")]
    Blog { id: String },
    #[at("/hosting/project/:name")]
    Project { name: String },
    #[at("/hosting/thesis")]
    Thesis,
    #[not_found]
    #[at("/hosting/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html
{
    match route
    {
        Route::Blog { id } => html! { <Page children={String::from(id)} /> },

        Route::Home | Route::Readme | Route::HomeBase => html! { <Page> <Readme /> </Page> },

        Route::Thesis => html! { <Page> <Thesis /> </Page> },

        Route::Project { name } => match name.as_str() {
            "floating_point_unit" => html! { <Page> <FloatingPointUnit/> </Page> },
            "vortexnote" => html! { <Page> <Vortexnote /> </Page> },
            "watchclean" => html! { <Page> <Watchclean /> </Page> },
            "only_vim" => html! { <Page> <OnlyVim /> </Page> },
            "ragtrace" => html! { <Page> <Ragtrace /> </Page> },
            "llm_debate" => html! { <Page> <LlmDebate /> </Page> },
            _ => html! { <div><h1>{ format!("Project: {}", name) }</h1></div> },
        },

        Route::NotFound => html! { <div><h1>{ "404 Not Found" }</h1></div> },
    }
}
