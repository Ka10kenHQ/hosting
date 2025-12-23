use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BodyProps {
    pub children: Children,
}

#[function_component(Body)]
pub fn body(props: &BodyProps) -> Html {
    html! {
        <div class="body-content">
            { &props.children }
        </div>
    }
}
