use yew::{Html, function_component, html};

#[function_component(Body)]
pub fn body() -> Html {
    html! {
        <>
            <section id="projects">
                <h2>{ "Projects" }</h2>
                <div class="projects-grid">
                    // TODO: project cards
                </div>
            </section>

            <section id="about">
                <h2>{ "About Me" }</h2>
                <p>{ "Short bio + skills." }</p>
            </section>

            <section id="contact">
                <h2>{ "Contact" }</h2>
                <p>{ "Email, GitHub, LinkedIn links." }</p>
            </section>
        </>
    }
}
