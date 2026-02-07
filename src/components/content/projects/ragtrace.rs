use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(Ragtrace)]
pub fn ragtrace() -> Html {
    html! {
        <ProjectPage
            title="RAGTrace"
            tagline="Visualization and diagnostic platform for RAG systems"
            description="Understand and refine retrieval-generation dynamics in RAG systems. Visualize how your RAG pipeline processes queries, retrieves context, and generates responses with interactive diagnostics."
            tech_stack={vec![
                "Python".to_string(),
                "Jupyter".to_string(),
                "Data Visualization".to_string(),
            ]}
            features={vec![
                "Multi-dimensional question analysis".to_string(),
                "Heatmap & force-directed graphs".to_string(),
                "Answer tracing & evidence analysis".to_string(),
                "Interactive prompt builder".to_string(),
            ]}
            github_url="https://github.com/Ka10ken1/RAGTrace"
        />
    }
}
