use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};

mod helper;

mod dom;
mod css;

type AttrMap = HashMap<String, Option<String>>;

pub fn init() -> Result<(), helper::Error> {
  println!("[init lib]");

  let mut f = File::open("index.html")
    .expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  

  dom::parse_dom(contents.to_string())
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
