use html_parser::{Dom, Node};

fn search_node(node: &Node, element_name: &String, nest: i32) -> Option<Node> {
    match node {
        Node::Element(element) => {
            if &element.name == element_name {
                return Some(Node::Element(element.clone()));
            }
            for child_node in element.children.iter() {
                let matched_node = search_node(child_node, element_name, nest + 1);
                if matched_node.is_some() {
                    return matched_node;
                }
            }
            None
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://mryhryki.com/").await?.text().await?;
    let dom = Dom::parse(&html).unwrap();
    let node: &Node = dom.children.first().unwrap();
    let title_node: Option<Node> = search_node(node, &String::from("title"), 1);

    println!("TITLE_NODE: {:#?}", title_node);
    Ok(())
}
