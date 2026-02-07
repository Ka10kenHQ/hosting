use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(LlmDebate)]
pub fn llm_debate() -> Html {
    html! {
        <ProjectPage
            title="LLM Debate"
            tagline="Multi-agent debate between LLMs for better reasoning"
            description="Research project exploring multi-agent debate between LLMs to improve factual accuracy and reasoning. Multiple language model instances debate their responses to arrive at better answers."
            tech_stack={vec![
                "Python".to_string(),
                "GPT-4".to_string(),
                "Claude".to_string(),
                "Multi-agent Systems".to_string(),
            ]}
            features={vec![
                "Multi-agent debate framework".to_string(),
                "Improved factual accuracy".to_string(),
                "Enhanced reasoning capabilities".to_string(),
                "Reduces hallucinations".to_string(),
            ]}
            github_url="https://github.com/Ka10ken1/llm-debate"
        />
    }
}
