use AttrMap;
use super::{Error, lexer::{TagContents, TagStatus}};

#[derive(Debug)]
struct Node {
  children: Vec<Node>,
  node_type: NodeType
}

#[derive(Debug)]
enum NodeType {
  Text(String),
  Element(ElementData)
}

#[derive(Debug)]
struct ElementData {
  tag_name: String,
  attributes: AttrMap
}

struct Parser {
  tags: Vec<TagContents>,
  current: usize
}

impl Parser {
  pub fn new(tags: Vec<TagContents>) -> Self {
    Self {
      tags,
      current: 0
    }
  }

  fn advance(&mut self) {
    self.current += 1;
  }

  pub fn get_node(&mut self) -> Result<Node, Error> {
    let self_point: *mut Self = self;

    let mut my_tag = match self.tags[self.current] {
      TagContents::Content(_) => return Err(Error("Node is content".to_string())),
      TagContents::Tag(ref tag) => tag
    };

    let mut children: Vec<Node> = Vec::new();

    if my_tag.status != TagStatus::SelfClose {
      loop {
        self.current += 1;

        let content = match self.tags[self.current] {
          TagContents::Content(ref s) => children.push(Node {
            children: Vec::new(),
            node_type: NodeType::Text(s.to_string())
          }),
          TagContents::Tag(ref tag) => {
            if tag.name == my_tag.name && tag.status == TagStatus::Close {
              break;
            }
            let child_tag = unsafe {
              (*self_point).get_node()?
            };
            children.push(child_tag);
          }
        };
      }
    }

    Ok(Node {
      children,
      node_type: NodeType::Element(ElementData {
        tag_name: my_tag.name.clone(),
        attributes: my_tag.attrs.clone()
      })
    })
  }
}

pub fn parse(tags: Vec<TagContents>) -> Result<(), Error> {

  let mut parser = Parser::new(tags);
  let root = parser.get_node()?;

  println!("{:?}", root);

  Ok(())
}