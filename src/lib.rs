use std::collections::HashMap;

mod dom;
mod css;

type AttrMap = HashMap<String, String>;

pub fn init() {
  println!("[init lib]");
  dom::parse_dom("<html>\n\t<body>\n\t\t<p style=\"color:red;\">\n\t\t\tHello there <b/>\n\t\t</p>\n\t</body>\n</html>".to_string());
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
