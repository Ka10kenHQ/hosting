use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(Ragtrace)]
pub fn ragtrace() -> Html {
    html! {
        <ProjectPage
            title="RAGTrace"
            tagline="Visualization and diagnostic platform for RAG systems"
            description="Understand and refine retrieval-generation dynamics in RAG systems. Visualize how your different RAG approaches processes and handles queries by detecting plagiarisms in code blocks"
            tools={vec![
                "Python".to_string(),
                "Jupyter".to_string(),
                "semantic embeddor".to_string(),
                "static embeddor".to_string(),
                "Rag".to_string(),
                "Hybrig RAG".to_string(),
                "Data Visualization".to_string(),
                "Gemini SDK".to_string()
            ]}
            features={vec![
                "Code plagiarism Detector".to_string(),
                "Question analysis".to_string(),
                "Answer tracing & evidence analysis".to_string()
            ]}
            github_url="https://github.com/Ka10ken1/RAGTrace"
            images={vec![
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/f1_combined.png".to_string(),
            ]}
        />
    }
}
