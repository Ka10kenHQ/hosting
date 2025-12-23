use crate::components::{body::Body, sidebar::Sidebar};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    html! {
        <div class="page-layout">
            <Sidebar/>
            <div class="main-content">
                <Body children={props.children.clone()} />
            </div>
        </div>
    }
}
