use yew::{Html, function_component, html};

use crate::components::content::projects::project_page::ProjectPage;

#[function_component(Watchclean)]
pub fn watchclean() -> Html 
{
    html!
    {
        <ProjectPage
            title="watchclean.tv"
            tagline="Ad-free blazingly fast local media collection manager"
            description="Watch your local movie and TV series collection without ads or streaming services. Built for speed and simplicity, it organizes your media library and provides a clean interface to browse and watch your content."
            tools={vec![
                "Go".to_string(),
                "Local Media Management".to_string(),
                "Ad-free Experience".to_string(),
            ]}
            features={vec![
                "Local movie/TV series collection organization".to_string(),
                "No ads or streaming dependencies".to_string(),
                "Blazingly fast performance".to_string(),
                "Clean, intuitive interface".to_string(),
            ]}
            github_url="https://github.com/Ka10kenHQ/watchclean.tv"
            images={vec![
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/series.jpg".to_string(),
            ]}
        />
    }
}
