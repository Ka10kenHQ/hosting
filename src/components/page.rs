use yew::{Html, function_component, html};

use crate::components::{body::Body, footer::Footer, header::Header, hero::Hero};

#[function_component]
pub fn Page() -> Html {
    html! {
        <>
            <Header/>
            <main>
                <Hero/>
                <Body/>
            </main>
            <Footer/>
        </>
    }
}
