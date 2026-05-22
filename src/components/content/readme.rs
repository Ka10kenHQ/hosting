use crate::routes::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Readme)]
pub fn readme() -> Html {
    html! {
        <div class="readme-container">
            <section class="hero">
                <p class="eyebrow">{"Developer Docs"}</p>
                <h1>{"Hi, I'm Mate Kopaliani"}</h1>
                <p class="hero-copy">
                    {"Computer Science student and software engineer (at least trying :) building back-end systems, \
                    AI tools, developer tooling, and low-level projects"}
                </p>
                <div class="hero-actions">
                    <a class="button-secondary" href="https://github.com/Ka10ken1" target="_blank">{"GitHub"}</a>
                </div>
            </section>

            <section class="dashboard-grid" aria-label="Overview">
                <article class="info-card intro-card">
                    <div class="card-label">{"Profile"}</div>
                    <h2>{"About"}</h2>
                    <p>
                        {"I enjoy building reliable services, terminal-centered workflows, \
                        system design experiments, and practical AI/ML projects."}
                    </p>
                    <p>
                        {"Currently working as a Software Engineer at Collision Vision, with \
                        previous back-end work around OCR, document processing, and translation."}
                    </p>
                    <div class="quick-links">
                        <a href="https://www.linkedin.com/in/mate-kopaliani-8838a7277" target="_blank">{"LinkedIn"}</a>
                        <a href="mailto:matekopaliani12@gmail.com">{"Email"}</a>
                    </div>
                </article>

                <article class="info-card releases-card">
                    <div class="card-label">{"Recent Work"}</div>
                    <h2>{"Releases"}</h2>
                    <div class="release-list">
                        <Link<Route> classes="release-row" to={Route::Project { name: "ragtrace".to_string() }}>
                            <span>{"2026-05"}</span>
                            <strong>{"RAGTrace"}</strong>
                            <em>{"diagnostics"}</em>
                        </Link<Route>>
                        <Link<Route> classes="release-row" to={Route::Project { name: "llm_debate".to_string() }}>
                            <span>{"2026-04"}</span>
                            <strong>{"LLM Debate"}</strong>
                            <em>{"agents"}</em>
                        </Link<Route>>
                        <Link<Route> classes="release-row" to={Route::Project { name: "watchclean".to_string() }}>
                            <span>{"2026-03"}</span>
                            <strong>{"watchclean.tv"}</strong>
                            <em>{"media"}</em>
                        </Link<Route>>
                        <Link<Route> classes="release-row" to={Route::Project { name: "only_vim".to_string() }}>
                            <span>{"2026-02"}</span>
                            <strong>{"OnlyVim"}</strong>
                            <em>{"tooling"}</em>
                        </Link<Route>>
                    </div>
                </article>

                <article class="info-card docs-card">
                    <div class="card-label">{"Notes"}</div>
                    <h2>{"Focus Areas"}</h2>
                    <ul>
                        <li>{"Distributed systems and back-end architecture"}</li>
                        <li>{"Math, AI/ML, and retrieval systems"}</li>
                        <li>{"Linux, terminals, and developer experience"}</li>
                        <li>{"Hardware-adjacent and low-level programming"}</li>
                    </ul>
                </article>
            </section>

            <section class="library-section">
                <div class="section-heading">
                    <p class="eyebrow">{"Library"}</p>
                    <h2>{"Projects"}</h2>
                </div>
                <div class="project-table">
                    <Link<Route> classes="project-row" to={Route::Project { name: "floating_point_unit".to_string() }}>
                        <span class="project-name">{"Floating Point Unit"}</span>
                        <span>{"SystemVerilog"}</span>
                        <span>{"IEEE 754 floating-point arithmetic unit"}</span>
                    </Link<Route>>
                    <Link<Route> classes="project-row" to={Route::Project { name: "jobless_ai".to_string() }}>
                        <span class="project-name">{"Jobless AI"}</span>
                        <span>{"Python / MongoDB"}</span>
                        <span>{"AI-powered job search automation"}</span>
                    </Link<Route>>
                    <Link<Route> classes="project-row" to={Route::Project { name: "llm_debate".to_string() }}>
                        <span class="project-name">{"LLM Debate"}</span>
                        <span>{"Python"}</span>
                        <span>{"Multi-agent debate system for better reasoning"}</span>
                    </Link<Route>>
                    <Link<Route> classes="project-row" to={Route::Project { name: "ragtrace".to_string() }}>
                        <span class="project-name">{"RAGTrace"}</span>
                        <span>{"Python / Jupyter"}</span>
                        <span>{"Different RAG approach analysis visualization and diagnostics"}</span>
                    </Link<Route>>
                    <Link<Route> classes="project-row" to={Route::Project { name: "watchclean".to_string() }}>
                        <span class="project-name">{"watchclean.tv"}</span>
                        <span>{"Go"}</span>
                        <span>{"Local media collection manager"}</span>
                    </Link<Route>>
                    <Link<Route> classes="project-row" to={Route::Project { name: "only_vim".to_string() }}>
                        <span class="project-name">{"OnlyVim"}</span>
                        <span>{"Lua / Neovim"}</span>
                        <span>{"Minimal Neovim distribution"}</span>
                    </Link<Route>>
                </div>
            </section>
        </div>
    }
}
