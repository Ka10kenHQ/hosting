use crate::components::body::Body;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps
{
    pub children: Children,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html
{
    html!
    {
        <div class="page-layout">
            <div class="main-content">
                <Body children={props.children.clone()} />
            </div>
        </div>
    }
}
