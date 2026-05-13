use my_site::app::App;
use yew::{Html, function_component, html};
use yew_router::BrowserRouter;

#[function_component(Main)]
pub fn main() -> Html
{
    html! {
        <BrowserRouter>
            <App />
        </BrowserRouter>
    }
}

fn main()
{
    yew::Renderer::<Main>::new().render();
}
