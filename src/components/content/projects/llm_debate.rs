use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(LlmDebate)]
pub fn llm_debate() -> Html {
    html! {
        <ProjectPage
            title="LLM Debate"
            tagline="Multi-agent debate between LLMs for better reasoning"
            description="Project exploring multi-agent debate between LLMs to improve factual accuracy and reasoning. Multiple language model instances debate their responses to arrive at better answers."
            tools={vec![
                "Python".to_string(),
                "Custom Message Queue".to_string(),
                "Asynchronous Processing".to_string(),
                "Multi-agent Systems".to_string(),
                "GPT, Claude, Gemini, Grok SDKs".to_string()
            ]}
            features={vec![
                "Multi-agent debate framework".to_string(),
                "Improved factual accuracy".to_string(),
                "Enhanced reasoning capabilities".to_string(),
                "Reduces hallucinations".to_string(),
            ]}
            github_url="https://github.com/Ka10ken1/llm-debate"
            images={vec![
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/detabe_arch.png".to_string(),
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/detabe_message_arch.png".to_string(),
            ]}
        />
    }
}
