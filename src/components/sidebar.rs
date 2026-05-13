use crate::models::sidebar_tree::*;
use crate::routes::Route;
use yew::events::KeyboardEvent;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Navigator;

fn file_icon_class(node: &Node) -> &'static str
{
    match node
    {
        Node::Dir(_, _) => "devicon-folder-plain",
        Node::File(name) => {
            if name.ends_with(".rs") {
                "devicon-rust-plain colored"
            } else if name.ends_with(".py") {
                "devicon-python-plain colored"
            } else if name.ends_with(".go") {
                "devicon-go-plain colored"
            } else if name.ends_with(".lua") {
                "devicon-lua-plain colored"
            } else if name.ends_with(".md") {
                "devicon-markdown-plain colored"
            } else if name.ends_with(".ipynb") {
                "devicon-jupyter-plain colored"
            } else if name.ends_with(".sv") {
                "devicon-verilog-plain colored"
            } else {
                "devicon-file-plain"
            }
        }
    }
}

fn markdown_page(name: &str) -> Option<Route>
{
    match name {
        "blog1.md" => Some(Route::Blog {
            id: "1".to_string(),
        }),
        "blog2.md" => Some(Route::Blog {
            id: "2".to_string(),
        }),
        "README.md" => Some(Route::Readme),
        _ => None,
    }
}

fn project_page(name: &str) -> Option<Route> 
{
    match name {
        "floating_point_unit.sv" => Some(Route::Project {
            name: "floating_point_unit".to_string(),
        }),
        "watchclean.tv.go" => Some(Route::Project {
            name: "watchclean".to_string(),
        }),
        "jobless_ai.py" => Some(Route::Project {
            name: "jobless_ai".to_string(),
        }),
        "OnlyVim.lua" => Some(Route::Project {
            name: "only_vim".to_string(),
        }),
        "ragtrace.ipynb" => Some(Route::Project {
            name: "ragtrace".to_string(),
        }),
        "llm-debate.py" => Some(Route::Project {
            name: "llm_debate".to_string(),
        }),
        _ => None,
    }
}

fn render_file_item(
    node: &Node,
    index: usize,
    selected_index: usize,
    onclick: Callback<usize>,
) -> Html
{
    let is_selected = index == selected_index;
    let onclick_clone = onclick.clone();

    html! {
        <div
            class={classes!("sidebar-item", if is_selected { "selected" } else { "" })}
            onclick={Callback::from(move |_| onclick_clone.emit(index))}
        >
            <span class={classes!("file-icon", file_icon_class(node))}></span>
            { node.name() }
        </div>
    }
}

fn handle_keydown(
    e: KeyboardEvent,
    selected_index: &UseStateHandle<usize>,
    nodes: &UseStateHandle<Vec<Node>>,
    current_path: &UseStateHandle<Vec<Node>>,
    navigator: Navigator,
) 
{
    let max_index = nodes.len().saturating_sub(1);

    match e.key().as_str() {
        "j" => selected_index.set((**selected_index + 1).min(max_index)),
        "k" => selected_index.set((**selected_index).saturating_sub(1)),
        "Enter" => {
            let onclick = make_onclick_callback(
                selected_index.clone(),
                nodes.clone(),
                current_path.clone(),
                navigator,
            );
            onclick.emit(**selected_index);
        }
        _ => {}
    }
}

fn navigate_to(navigator: &yew_router::navigator::Navigator, route: Route) 
{
    navigator.push(&route);
}

fn make_onclick_callback(
    selected_index: UseStateHandle<usize>,
    nodes: UseStateHandle<Vec<Node>>,
    current_path: UseStateHandle<Vec<Node>>,
    navigator: Navigator,
) -> Callback<usize>
{
    Callback::from(move |index: usize|
    {
        selected_index.set(index);
        let node = &nodes[index];

        match node
        {
            Node::File(name) if name == "../" =>
            {
                let mut stack = (*current_path).clone();
                stack.pop();

                let parent_nodes = if stack.is_empty() {
                    root_nodes()
                } else {
                    stack.last().unwrap().get_children()
                };

                current_path.set(stack);
                nodes.set(parent_nodes);
            }
            Node::Dir(_, _) =>
            {
                let mut stack = (*current_path).clone();
                stack.push(node.clone());
                current_path.set(stack);
                nodes.set(node.get_children());
            }
            Node::File(name) =>
            {
                if let Some(page) = project_page(name)
                {
                    navigate_to(&navigator, page);
                } else if let Some(page) = markdown_page(name)
                {
                    navigate_to(&navigator, page);
                } else
                {
                    web_sys::console::log_1(&format!("File selected: {}", name).into());
                }
            }
        }
    })
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html
{
    let nodes = use_state(|| root_nodes());
    let selected_index = use_state(|| 0);
    let current_path = use_state(|| vec![]);
    let navigator = use_navigator().unwrap();

    let on_keydown =
    {
        let selected_index_clone = selected_index.clone();
        let nodes_clone = nodes.clone();
        let current_path_clone = current_path.clone();
        let navigator_for_keydown = navigator.clone();

        Callback::from(move |e: KeyboardEvent|
        {
            let selected_index_inner = selected_index_clone.clone();
            let nodes_inner = nodes_clone.clone();
            let current_path_inner = current_path_clone.clone();
            handle_keydown(
                e,
                &selected_index_inner,
                &nodes_inner,
                &current_path_inner,
                navigator_for_keydown.clone(),
            )
        })
    };

    let onclick_item = make_onclick_callback(
        selected_index.clone(),
        nodes.clone(),
        current_path.clone(),
        navigator.clone(),
    );

    html! 
    {
        <aside class="sidebar" tabindex="0" onkeydown={on_keydown}>
            { for nodes.iter().enumerate().map(|(i, node)| {
                render_file_item(node, i, *selected_index, onclick_item.clone())
            })
            }
            <div class="sidebar-status">
                { ":!sudo rm -rf /" }
            </div>
        </aside>
    }
}
