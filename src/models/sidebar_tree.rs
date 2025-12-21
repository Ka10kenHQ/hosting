#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    File(String),
    Dir(String, Vec<Node>),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::File(name) => name,
            Node::Dir(name, _) => name,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Node::Dir(_, _))
    }

    pub fn get_children(&self) -> Vec<Node> {
        match self {
            Node::File(_) => vec![],
            Node::Dir(_, children) => children.clone(),
        }
    }
}

pub fn root_nodes() -> Vec<Node> {
    vec![
        Node::File("../".to_string()),
        Node::File("./".to_string()),
        Node::Dir(
            "projects/".to_string(),
            vec![
                Node::File("../".to_string()),
                Node::File("./".to_string()),
                Node::File("watchclean.tv.go".to_string()),
                Node::File("jobless_ai.py".to_string()),
                Node::File("OnlyVim.lua".to_string()),
                Node::File("ragtrace.ipynb".to_string()),
            ],
        ),
        Node::Dir(
            "blogs/".to_string(),
            vec![
                Node::File("../".to_string()),
                Node::File("./".to_string()),
                Node::File("blog1.md".to_string()),
                Node::File("blog2.md".to_string()),
            ],
        ),
        Node::File("README.md".to_string()),
    ]
}
