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
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
        <Page>
                   <div class="vim-startup">
                       <pre>
                           {r#"
                         Hi!

                 I'm Mate also known as (achir)

                 Software Engineer focused on
                 backend and AI related topics.

                 Mostly working with
                 Go - C# - Rust - Lua - Java - Python

                 4th-year Computer Science student
                 with a minor in Mathematics

                 Select a file on the left
                 to explore projects and source code

                 Thanks for stopping by
                        "#}
                       </pre>
                   </div>
               </Page>
           },
        Route::Blog { id } => html! { <Page children={String::from(id)} /> },
        Route::About => html! { <Page> <Readme /> </Page> },
        Route::NotFound => html! { <div><h1>{ "404 Not Found" }</h1></div> },
    }
}
