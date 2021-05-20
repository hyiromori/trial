use html_parser::{Dom, Node};

fn search_node(node: &Node, tag_name: &String) -> Option<Node> {
    match node {
        Node::Element(element) => {
            if &element.name == tag_name {
                return Some(Node::Element(element.clone()));
            }
            for child_node in element.children.iter() {
                let matched_node = search_node(child_node, tag_name);
                if matched_node.is_some() {
                    return matched_node;
                }
            }
            None
        }
        _ => None,
    }
}

fn text_content(node: &Node) -> String {
    match node {
        Node::Text(text) => {
            text.to_string()
        },
        Node::Element(element) => {
            let mut text = String::from("");
            for child_node in element.children.iter() {
                text.push_str(&text_content(child_node));
            }
            text
        },
        Node::Comment(_) => String::from("")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://mryhryki.com/").await?.text().await?;
    let dom = Dom::parse(&html).unwrap();
    let node: &Node = dom.children.first().unwrap();
    let title_node: Option<Node> = search_node(node, &String::from("title"));

    match title_node {
        Some(node) => {
            println!("Title[{}]", text_content(&node));
        },
        None => (),
    }
    Ok(())
}
