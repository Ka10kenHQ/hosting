use yew::{Html, function_component, html};

#[function_component]
pub fn Hero() -> Html {
    html! {
        <section id="hero">
            <h2>{ "Hello, I'm a Backend Engineer" }</h2>
            <p>{ "Go & Rust enthusiast building clean, fast, maintainable software." }</p>
            <button class="button">{ "See Projects" }</button>
        </section>
    }
}
