use crate::routes::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Readme)]
pub fn readme() -> Html {
    html! {
        <div class="readme-container">

            <section class="hero">
                <h2>{"Hi, I'm Mate Kopaliani"}</h2>
                <p class="hero-copy">
                    {"Computer Science graduate and software engineer. building back-end systems, \
                    AI tools, developer tooling, and low-level stuff"}
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
                        {"I'm 20 years old and I enjoy building reliable services, terminal-centered workflows, \
                        system design experiments, and practical AI/ML projects."}
                    </p>
                    <div class="quick-links">
                        <a href="https://www.linkedin.com/in/mate-kopaliani-8838a7277" target="_blank">{"LinkedIn"}</a>
                        <a href="mailto:matekopaliani12@gmail.com">{"Email"}</a>
                    </div>
                </article>
            </section>

            <section class="resume-section" aria-label="Background">
                <div class="section-heading">
                    <h2>{"Experience"}</h2>
                </div>

                <div class="resume-list">
                    <article class="resume-entry">
                        <div class="resume-entry__meta">
                            <span class="resume-entry__title">{"Software Engineer"}</span>
                            <span class="resume-entry__org">
                                <a href="https://www.cvcertified.com/" target="_blank" rel="noopener noreferrer">{"Collision Vision"}</a>
                            </span>
                        </div>
                        <p class="resume-entry__summary">
                            {"Working on back-end systems and production software with a focus on reliability, practical tooling, and real-world product constraints."}
                        </p>
                    </article>

                    <article class="resume-entry">
                        <div class="resume-entry__meta">
                            <span class="resume-entry__title">{"Back-end Engineering Intern"}</span>
                            <span class="resume-entry__org">
                                <a href="https://api24.ge" target="_blank" rel="noopener noreferrer">{"api24.ge"}</a>
                            </span>
                        </div>
                        <p class="resume-entry__summary">
                            {"Built systems around OCR, document processing, and translation workflows."}
                        </p>
                    </article>

                    <article class="resume-entry">
                        <div class="resume-entry__meta">
                            <span class="resume-entry__title">{"Linear Algebra Student Tutor"}</span>
                            <span class="resume-entry__org">{"Kutaisi International University"}</span>
                        </div>
                        <p class="resume-entry__summary">
                            {"Held weekly meetings with peer students, reviewed previous-week material, and helped reinforce core linear algebra topics."}
                        </p>
                    </article>
                </div>
            </section>

            <section class="resume-section" aria-label="Education">
                <div class="section-heading">
                    <h2>{"Education"}</h2>
                </div>

                <div class="resume-list">
                    <article class="resume-entry">
                        <div class="resume-entry__meta">
                            <span class="resume-entry__title">{"B.S. in Computer Science, Minor in Mathematics"}</span>
                            <span class="resume-entry__org">
                                <a href="https://kiu.edu.ge" target="_blank" rel="noopener noreferrer">{"Kutaisi International University"}</a>
                            </span>
                        </div>
                        <p class="resume-entry__summary">
                            {"Graduated with a 3.6/4.0 GPA and an average score of 93/100."}
                        </p>
                    </article>
                </div>
            </section>

            <section class="resume-section" aria-label="Bachelor Thesis">
                <div class="section-heading">
                    <h2>{"Bachelor Thesis"}</h2>
                </div>

                <div class="resume-list">
                    <article class="resume-entry">
                        <div class="resume-entry__meta">
                            <span class="resume-entry__title">{"Floating Point Unit"}</span>
                            <span class="resume-entry__org">{"Rated 99/100"}</span>
                        </div>
                        <p class="resume-entry__summary">
                            {"Bachelor thesis, co-authored with Zura Kajaia, focused on designing and implementing an IEEE 754 floating-point arithmetic unit. Supervisors: "}
                            <a
                                href="https://research.com/u/wolfgang-j-paul"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {"Wolfgang J. Paul"}
                            </a>
                            {" and "}
                            <a
                                href="https://www.linkedin.com/in/waltertichy/"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {"Walter Tichy"}
                            </a>
                            {"."}
                        </p>
                        <div class="quick-links">
                            <a href="/hosting/thesis.pdf" download="thesis.pdf">{"Download thesis"}</a>
                        </div>
                    </article>
                </div>
            </section>

            <section class="library-section">

                <div class="section-heading">
                    <p class="eyebrow">{"Library"}</p>
                    <h2>{"Projects"}</h2>
                </div>

                <div class="project-table">

                    <Link<Route> classes="project-row" to={Route::Project { name: "floating_point_unit".to_string() }}>
                        <span class="project-name">{"Floating Point Unit"}</span>
                        <span>{"IEEE 754 floating-point arithmetic unit"}</span>
                    </Link<Route>>

                    <Link<Route> classes="project-row" to={Route::Project { name: "vortexnote".to_string() }}>
                        <span class="project-name">{"VortexNote"}</span>
                        <span>{"File management platform with AI-powered document search"}</span>
                    </Link<Route>>

                    <Link<Route> classes="project-row" to={Route::Project { name: "llm_debate".to_string() }}>
                        <span class="project-name">{"LLM Debate"}</span>
                        <span>{"Multi-agent debate system for better reasoning"}</span>
                    </Link<Route>>

                    <Link<Route> classes="project-row" to={Route::Project { name: "ragtrace".to_string() }}>
                        <span class="project-name">{"RAGTrace"}</span>
                        <span>{"Different RAG approach analysis visualization and diagnostics"}</span>
                    </Link<Route>>

                    <Link<Route> classes="project-row" to={Route::Project { name: "watchclean".to_string() }}>
                        <span class="project-name">{"watchclean.tv"}</span>
                        <span>{"Local media collection manager"}</span>
                    </Link<Route>>

                    <Link<Route> classes="project-row" to={Route::Project { name: "only_vim".to_string() }}>
                        <span class="project-name">{"OnlyVim"}</span>
                        <span>{"Minimal Neovim distribution"}</span>
                    </Link<Route>>

                </div>
            </section>
        </div>
    }
}
