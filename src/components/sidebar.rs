use crate::models::sidebar_tree::*;
use yew::events::KeyboardEvent;
use yew::prelude::*;

fn file_icon_class(node: &Node) -> &'static str {
    match node {
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
            } else {
                "devicon-file-plain"
            }
        }
    }
}

fn github_url(name: &str) -> Option<&'static str> {
    match name {
        "watchclean.tv.go" => Some("https://github.com/Ka10kenHQ/watchclean.tv"),
        "jobless_ai.py" => Some("https://github.com/Ka10kenHQ/Jobless-AI"),
        "OnlyVim.lua" => Some("https://github.com/Ka10kenHQ/OnlyVim"),
        "ragtrace.ipynb" => Some("https://github.com/Ka10ken1/RAGTrace"),
        _ => None,
    }
}

fn markdown_page(name: &str) -> Option<&'static str> {
    match name {
        "blog1.md" => Some("/blog/blog1"),
        "blog2.md" => Some("/blog/blog2"),
        "README.md" => Some("/about"),
        _ => None,
    }
}

fn render_file_item(
    node: &Node,
    index: usize,
    selected_index: usize,
    onclick: Callback<usize>,
) -> Html {
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
) {
    let max_index = nodes.len().saturating_sub(1);

    match e.key().as_str() {
        "j" => selected_index.set((**selected_index + 1).min(max_index)),
        "k" => selected_index.set((**selected_index).saturating_sub(1)),
        "Enter" => {
            let onclick =
                make_onclick_callback(selected_index.clone(), nodes.clone(), current_path.clone());
            onclick.emit(**selected_index);
        }
        _ => {}
    }
}

fn make_onclick_callback(
    selected_index: UseStateHandle<usize>,
    nodes: UseStateHandle<Vec<Node>>,
    current_path: UseStateHandle<Vec<Node>>,
) -> Callback<usize> {
    Callback::from(move |index: usize| {
        selected_index.set(index);
        let node = &nodes[index];

        match node {
            Node::File(name) if name == "../" => {
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
            Node::Dir(_, _) => {
                let mut stack = (*current_path).clone();
                stack.push(node.clone());
                current_path.set(stack);
                nodes.set(node.get_children());
            }
            Node::File(name) => {
                if let Some(url) = github_url(name) {
                    if let Some(win) = web_sys::window() {
                        let _ = win.location().set_href(url);
                    }
                } else if let Some(page) = markdown_page(name) {
                    if let Some(win) = web_sys::window() {
                        let _ = win.location().set_href(page);
                    }
                } else {
                    web_sys::console::log_1(&format!("File selected: {}", name).into());
                }
            }
        }
    })
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let nodes = use_state(|| root_nodes());
    let selected_index = use_state(|| 0);
    let current_path = use_state(|| vec![]);

    let on_keydown = {
        let selected_index_clone = selected_index.clone();
        let nodes_clone = nodes.clone();
        let current_path_clone = current_path.clone();

        Callback::from(move |e: KeyboardEvent| {
            handle_keydown(e, &selected_index_clone, &nodes_clone, &current_path_clone)
        })
    };

    let onclick_item =
        make_onclick_callback(selected_index.clone(), nodes.clone(), current_path.clone());

    html! {
        <aside class="sidebar" tabindex="0" onkeydown={on_keydown}>
            { for nodes.iter().enumerate().map(|(i, node)| {
                render_file_item(node, i, *selected_index, onclick_item.clone())
            })
            }
            <div class="sidebar-status">
                { ":!rm -rf ~/saved" }
            </div>
        </aside>
    }
}
