use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(JoblessAi)]
pub fn jobless_ai() -> Html {
    html! {
        <ProjectPage
            title="Jobless AI"
            tagline="AI-powered job search automation tool"
            description="Automate your job search process with AI assistance. This tool helps you find and apply to jobs tailored to your skills, making the job hunting process faster and more efficient."
            tech_stack={vec![
                "Python".to_string(),
                "AI/ML".to_string(),
                "MCP".to_string(),
                "MongoDB".to_string(),
            ]}
            features={vec![
                "Automated job searching".to_string(),
                "AI-tailored applications".to_string(),
                "Personalized job recommendations".to_string(),
                "Streamlined application process".to_string(),
            ]}
            github_url="https://github.com/Ka10kenHQ/Jobless-AI"
        />
    }
}
