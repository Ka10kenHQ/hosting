use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(OnlyVim)]
pub fn only_vim() -> Html {
    html! {
        <ProjectPage
            title="OnlyVim"
            tagline="Minimal, sane, and ready-to-hack Neovim distribution"
            description="A lightweight Neovim distribution to help more people get into Neovim without the usual pain. Designed to be minimal, easy to customize, and beginner-friendly while maintaining the power of Vim."
            tools={vec![
                "Lua".to_string(),
                "Neovim".to_string()
            ]}
            features={vec![
                "Minimal and lightweight".to_string(),
                "Easy to customize and hack".to_string(),
                "Beginner-friendly Neovim experience".to_string(),
                "Sane defaults out of the box".to_string(),
            ]}
            github_url="https://github.com/Ka10kenHQ/OnlyVim"
            images={vec![
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/onlyvim.png".to_string(),
            ]}
        />
    }
}
