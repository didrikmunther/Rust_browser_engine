use super::helper;
use helper::Error;

mod lexer;
mod parser;

use self::parser::{Stylesheet, Declaration};

pub fn parse_stylesheet(query: String) -> Result<Stylesheet, Error> {
  println!("[init css]");

  let lexed = lexer::lex(query)?;
  let stylesheet = parser::parse_stylesheet(lexed)?;

  println!("style: {:?}", stylesheet);

  Ok(stylesheet)
}

pub fn parse_declarations(query: String) -> Result<Vec<Declaration>, Error> {
  let lexed = lexer::lex(query)?;
  Ok(parser::parse_declarations(lexed)?)
}