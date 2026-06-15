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

    #[prop_or_default]
    pub images: Vec<String>,
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectPageProps) -> Html {
    let has_images = !props.images.is_empty();
    let projects = [
        ("ragtrace", "RAGTrace"),
        ("llm_debate", "LLM Debate"),
        ("watchclean", "watchclean.tv"),
        ("only_vim", "OnlyVim"),
        ("vortexnote", "VortexNote"),
        ("floating_point_unit", "Floating Point Unit"),
        ("jobless_ai", "Jobless AI"),
    ];

    html! {
        <div class="project-container">
            <header class="project-hero">
                <h1 class="project-title">{&props.title}</h1>
                <p class="project-tagline">{&props.tagline}</p>
            </header>

            <nav class="project-switcher" aria-label="Browse projects">
                <span class="project-switcher__label">{"Browse Projects"}</span>
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

            {if has_images {
                html! {
                    <section class="project-gallery">
                        <div class="gallery-grid">
                            {props.images.iter().enumerate().map(|(i, img)| {
                                let loading = if i == 0 { "eager" } else { "lazy" };
                                let fetchpriority = if i == 0 { "high" } else { "low" };

                                html! {
                                    <div class="gallery-item" key={i}>
                                        <img
                                            src={img.clone()}
                                            alt={format!("{} screenshot {}", props.title, i + 1)}
                                            loading={loading}
                                            decoding="async"
                                            fetchpriority={fetchpriority}
                                        />
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                    </section>
                }
            } else {
                html! {}
            }}

            <section class="project-section">
                <p class="project-description">{&props.description}</p>
                <div class="tech-stack">
                    <h3>{"Tools"}</h3>
                    <div class="tech-badges">
                        {props.tools.iter().map(|tech| {
                            html! {
                                <span class="tech-badge">{tech}</span>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </section>

            <section class="project-section">
                <h3>{"Features"}</h3>
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
            <a href={props.github_url.clone()} target="_blank" class="github-cta">
                // <span class="github-icon">{"⚡"}</span>
                {"View on GitHub"}
            </a>
        </div>
    }
}
