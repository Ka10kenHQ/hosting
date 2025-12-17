use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let collapsed = use_state(|| false);

    let toggle = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };

    let sidebar_class = if *collapsed {
        "sidebar collapsed"
    } else {
        "sidebar"
    };

    html! {
        <>
            <nav class={sidebar_class}>
                <a href="#hero">{ "home" }</a>
                <a href="#blogs">{ "blogs" }</a>
            </nav>
            <button onclick={toggle} class="sidebar-toggle">
                { if *collapsed { "→" } else { "←" } }
            </button>
        </>
    }
}
