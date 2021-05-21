use html_parser::{Dom, Node};

fn search_nodes(node: &Node, tag_name: &String) -> Vec<Node> {
    match node {
        Node::Element(element) => {
            if &element.name == tag_name {
                return vec![Node::Element(element.clone())];
            }
            element
                .children
                .iter()
                .map(|child_node| -> Vec<Node> { search_nodes(child_node, tag_name) })
                .fold(vec![], |mut base, nodes| -> Vec<Node> {
                    for node in nodes {
                        base.push(node);
                    }
                    base
                })
        }
        _ => vec![],
    }
}

fn search_node(node: &Node, tag_name: &String) -> Option<Node> {
    match search_nodes(node, tag_name).first() {
        Some(node) => Some(node.clone()),
        None => None,
    }
}

fn text_content(node: &Node) -> String {
    match node {
        Node::Text(text) => text.to_string(),
        Node::Element(element) => {
            let mut text = String::from("");
            for child_node in element.children.iter() {
                text.push_str(&text_content(child_node));
            }
            text
        }
        Node::Comment(_) => String::from(""),
    }
}

fn site_image_url(metas: &Vec<Node>) -> Option<String> {
    let mut og_image: String = String::from("");
    let mut twitter_image: String = String::from("");

    for meta in metas {
        match meta {
            Node::Element(element) => {
                let property: &Option<String> = element.attributes.get("property").unwrap_or(&None);
                let name: &Option<String> = element.attributes.get("name").unwrap_or(&None);

                if property == &Some(String::from("og:image")) {
                    match element.attributes.get("content").unwrap_or(&None) {
                        Some(content) => og_image = String::from(content),
                        None => (),
                    }
                } else if name == &Some(String::from("twitter:image")) {
                    match element.attributes.get("content").unwrap_or(&None) {
                        Some(content) => twitter_image = String::from(content),
                        None => (),
                    }
                }
            }
            _ => (),
        }
    }

    if og_image != String::from("") {
        Some(og_image)
    } else if twitter_image != String::from("") {
        Some(twitter_image)
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://zenn.dev/mryhryki/articles/2021-05-16-update-aws-lambda-fast";
    let html = reqwest::get(url).await?.text().await?;
    let dom = Dom::parse(&html).unwrap();
    let head: Node = search_node(dom.children.first().unwrap(), &String::from("head")).unwrap();
    let metas: Vec<Node> = search_nodes(&head, &String::from("meta"));

    let title: String = match search_node(&head, &String::from("title")) {
        Some(node) => text_content(&node),
        None => String::from(""),
    };
    let image_url: String = match site_image_url(&metas) {
        Some(url) => url,
        None => String::from(""),
    };

    println!("Title[{}]", title);
    println!("Image URL[{}]", image_url);
    Ok(())
}
