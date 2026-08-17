use crate::routes::Route;
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct ProjectPageProps {
    pub title: String,
    pub tagline: String,
    pub description: String,
    pub tools: Vec<String>,
    pub features: Vec<String>,
    pub github_url: String,
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectPageProps) -> Html {
    let projects = [
        ("floating_point_unit", "Floating Point Unit"),
        ("vortexnote", "VortexNote"),
        ("llm_debate", "LLM Debate"),
        ("ragtrace", "RAGTrace"),
        ("watchclean", "watchclean.tv"),
        ("only_vim", "OnlyVim"),
    ];

    html! {
        <div class="project-container">
            <header class="project-hero">
                <h1 class="project-title">{&props.title}</h1>
                <p class="project-tagline">{&props.tagline}</p>
            </header>

            <nav class="project-switcher" aria-label="Browse projects">
                <div class="project-switcher__links">
                    {projects.iter().map(|(slug, title)| {
                        let is_current = *title == props.title;

                        html! {
                            <Link<Route>
                                classes={classes!("project-switcher__link", is_current.then_some("is-current"))}
                                to={Route::Project { name: (*slug).to_string() }}
                            >
                                {title}
                            </Link<Route>>
                        }
                    }).collect::<Html>()}
                </div>
            </nav>

            <section class="project-section project-section--lead">
                <div class="project-section__label">{"Overview"}</div>
                <p class="project-description">{&props.description}</p>
            </section>

            <section class="project-details-grid">
                <section class="project-section">
                    <div class="project-section__label">{"Repository"}</div>
                    <a href={props.github_url.clone()} target="_blank" class="github-cta" rel="noopener noreferrer">
                        {"View on GitHub"}
                    </a>
                </section>

                <section class="project-section">
                    <div class="project-section__label">{"Tools"}</div>
                    <div class="tech-badges">
                        {props.tools.iter().map(|tech| {
                            html! {
                                <span class="tech-badge">{tech}</span>
                            }
                        }).collect::<Html>()}
                    </div>
                </section>

                <section class="project-section">
                    <div class="project-section__label">{"Features"}</div>
                    <ul class="feature-list">
                        {props.features.iter().map(|feature| {
                            html! {
                                <li class="feature-item">
                                    <span class="feature-bullet">{"→"}</span>
                                    <span>{feature}</span>
                                </li>
                            }
                        }).collect::<Html>()}
                    </ul>
                </section>
            </section>
        </div>
    }
}
