mod dom;
mod css;

pub fn init() {
  print!("hello");
  dom::parse_dom("test".to_string());
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
