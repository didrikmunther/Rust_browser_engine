#![macro_escape]

#[macro_use]
macro_rules! map(
  { $($key:expr => $value:expr),+ } => {
    {
      let mut m = ::std::collections::HashMap::new();
        $(
          m.insert($key, $value);
        )+
      m
    }
  };
);

#[derive(Debug)]
pub struct Error(pub String);

#[derive(Debug)]
pub enum PreLexed {
  String(String),
  Rest(String)
}

pub fn pre_lex(query: String) -> Result<Vec<PreLexed>, Error> {
  let mut result: Vec<PreLexed> = Vec::new();
  let mut buf: String = String::new();
  let mut is_string = false;

  for (_i, c) in query.chars().enumerate() {
    match c {
      '"' => {
        if is_string {
          result.push(PreLexed::String(buf));
          buf = String::new();
          is_string = false;
        } else {
          result.push(PreLexed::Rest(buf));
          buf = String::new();
          is_string = true;
        }
      },
      _ => {
        buf.push(c);
      }
    }
  }

  result.push(PreLexed::Rest(buf));

  Ok(result)
}