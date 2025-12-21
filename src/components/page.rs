use yew::{Callback, Html, function_component, html, use_state};

use crate::{
    components::{body::Body, sidebar::Sidebar},
    models::sidebar_tree::Node,
};

#[function_component]
pub fn Page() -> Html {
    let selected_node = use_state(|| None::<Node>);

    let on_select_node = {
        let selected_node = selected_node.clone();
        Callback::from(move |node: Node| {
            selected_node.set(Some(node));
        })
    };

    html! {
        <div class="page-layout">
            <Sidebar on_select={on_select_node.clone()} />
            <div class="main-content">
                <Body selected_node={(*selected_node).clone()} />
            </div>
        </div>
    }
}
