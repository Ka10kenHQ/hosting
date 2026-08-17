use yew::{function_component, html, Html};

#[function_component(Thesis)]
pub fn thesis() -> Html {
    html! {
        <div class="project-container thesis-page">
            <header class="project-hero">
                <h1 class="project-title">{"Bachelor Thesis"}</h1>
                <p class="project-tagline">
                    {"Floating Point Unit · rated 99/100"}
                </p>
            </header>

            <section class="project-section project-section--lead">
                <div class="project-section__label">{"Document"}</div>
                <p class="project-description">
                    {"Embedded PDF viewer for the thesis document. If the viewer does not render in your browser, open the PDF directly."}
                </p>
            </section>

            <section class="thesis-viewer-section">
                <iframe
                    class="thesis-viewer"
                    src="/thesis.pdf"
                    title="Bachelor thesis PDF"
                />
            </section>

            <div class="quick-links">
                <a href="/thesis.pdf" target="_blank" rel="noopener noreferrer">{"Open PDF"}</a>
                <a href="/thesis.pdf" download="thesis.pdf">{"Download PDF"}</a>
            </div>
        </div>
    }
}
