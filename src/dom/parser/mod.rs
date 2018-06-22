use AttrMap;

struct Node {
  children: Vec<Node>,
  node_type: NodeType
}

enum NodeType {
  Text(String),
  Element(ElementData)
}

struct ElementData {
  tag_name: String,
  attributes: AttrMap
}