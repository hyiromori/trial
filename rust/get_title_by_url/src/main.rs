use html_parser::{Dom, Node};

fn search_node(node: &Node, element_name: &String, nest: i32) -> Option<Node> {
    match node {
        Node::Element(element) => {
            if &element.name == element_name {
                Some(Node::Element(element.clone()))
            } else {
                let match_node: Option<&Node> = element.children.iter().find(|child_node| {
                    let match_node = search_node(&child_node, element_name, nest + 1);
                    match match_node {
                        Some(_) => true,
                        None => false,
                    }
                });
                let result = match match_node {
                    Some(n) => Some(n.clone()),
                    None => None,
                };
                println!("RESULT: {}, {:#?}", nest, result);
                result
            }
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://mryhryki.com/").await?.text().await?;
    let dom = Dom::parse(&html).unwrap();
    let node: &Node = dom.children.first().unwrap();
    let _title_node: Option<Node> = search_node(node, &String::from("title"), 1);

    // println!("TITLE_NODE: {:#?}", title_node);
    Ok(())
}
