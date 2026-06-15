use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(Vortexnote)]
pub fn vortexnote() -> Html
{
    html!
    {
        <ProjectPage
            title="VortexNote"
            tagline="File management platform with AI-powered document search"
            description="A full-stack file management system for organizing projects and documents. Each project supports multiple notes and documents with upload, download, tagging, and cross-referencing. Features a real-time dashboard with project and activity overview, notification streaming, and an AI embedding service for semantic content search across all documents."
            tools={vec![
                "Rust".to_string(),
                "Go".to_string(),
                "Python".to_string(),
                "Svelte".to_string(),
                "RabbitMQ".to_string(),
                "AI Embeddings".to_string(),
            ]}
            features={vec![
                "Project-based document organization with tagging and cross-references".to_string(),
                "Real-time dashboard with activity overview and metrics".to_string(),
                "Notification streaming via server-sent events".to_string(),
                "AI-powered semantic search across document contents".to_string(),
                "Role-based access control via Guardian auth service".to_string(),
                "Reverse-proxy gateway written in Go".to_string(),
                "Asynchronous background processing with RabbitMQ".to_string(),
            ]}
            github_url="https://vortexnote.com"
            images={vec![
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/dashboard.png".to_string(),
            ]}
        />
    }
}
