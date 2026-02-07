use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ProjectPageProps {
    pub title: String,
    pub tagline: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub features: Vec<String>,
    pub github_url: String,
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectPageProps) -> Html {
    html! {
        <div class="project-container">
            <header class="project-hero">
                <h1 class="project-title">{&props.title}</h1>
                <p class="project-tagline">{&props.tagline}</p>
            </header>
            <section class="project-section">
                <p class="project-description">{&props.description}</p>

                <div class="tech-stack">
                    <h3>{"Tech Stack"}</h3>
                    <div class="tech-badges">
                        {props.tech_stack.iter().map(|tech| {
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
                <span class="github-icon">{"⚡"}</span>
                {"View on GitHub"}
            </a>
        </div>
    }
}
