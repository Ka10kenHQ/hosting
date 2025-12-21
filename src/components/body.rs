use crate::models::sidebar_tree::Node;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BodyProps {
    pub selected_node: Option<Node>,
}

#[function_component(Body)]
pub fn body(props: &BodyProps) -> Html {
    if let Some(node) = &props.selected_node {
        match node {
            Node::File(name) if name.ends_with(".rs") => html! {
                <div class="body-section">
                    <h2>{ format!("Rust file: {}", name) }</h2>
                </div>
            },
            Node::File(name) if name.ends_with(".py") => html! {
                <div class="body-section">
                    <h2>{ format!("Python file: {}", name) }</h2>
                </div>
            },
            Node::File(name) if name.ends_with(".md") => html! {
                <div class="body-section">
                    <h2>{ format!("Markdown file: {}", name) }</h2>
                </div>
            },
            Node::File(name) if name.ends_with(".ipynb") => html! {
                <div class="body-section">
                    <h2>{ format!("Jupyter Notebook: {}", name) }</h2>
                </div>
            },
            Node::File(name) => html! {
                <div class="body-section">
                    <h2>{ format!("File: {}", name) }</h2>
                </div>
            },
            Node::Dir(_, _) => html! {
                <div class="body-section">
                    <section class="body-section__projects">
                        <h2 class="body-section__title">{ "Projects" }</h2>
                        <div class="body-section__grid">
                        </div>
                    </section>

                    <section class="body-section__about">
                        <h2 class="body-section__title">{ "About Me" }</h2>
                        <p class="body-section__text">{ "Short bio + skills." }</p>
                    </section>

                    <section class="body-section__contact">
                        <h2 class="body-section__title">{ "Contact" }</h2>
                        <p class="body-section__text">{ "Email, GitHub, LinkedIn links." }</p>
                    </section>
                </div>
            },
        }
    } else {
        html! {
        <div class="vim-startup">
            <pre>
            {r#"
                             Hi!

                 I'm Mate also known as (achir)

                 Software Engineer focused on
                 backend and AI related topics.
                
                 Mostly working with
                 Go - C# - Rust - Lua - Java - Python

                 4th-year Computer Science student
                 with a minor in Mathematics

                 Select a file on the left
                 to explore projects and source code

                 Thanks for stopping by
                "#}
            </pre>
        </div>
        }
    }
}
