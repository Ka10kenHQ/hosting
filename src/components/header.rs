use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let collapsed = use_state(|| false);

    let toggle = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };

    let sidebar_class = if *collapsed {
        "header-sidebar sidebar-collapsed"
    } else {
        "header-sidebar"
    };

    html! {
        <>
            <aside class={sidebar_class}>
                <a class="header-sidebar__link" href="#hero">{ "Home" }</a>
                <a class="header-sidebar__link" href="#projects">{ "Projects" }</a>
                <a class="header-sidebar__link" href="#about">{ "About" }</a>
                <a class="header-sidebar__link" href="#contact">{ "Contact" }</a>
            </aside>
            <button onclick={toggle} class="header-sidebar__toggle">
                { if *collapsed { "→" } else { "←" } }
            </button>
        </>
    }
}
