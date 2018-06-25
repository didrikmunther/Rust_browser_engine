use super::helper;
use helper::Error;

mod lexer;
mod parser;

pub fn parse_dom(query: String) -> Result<(), Error> {
  let lexed = lexer::lex(query)?;

  // println!("lexed: {:?}", lexed);

  let parsed = parser::parse(lexed)?;

  println!("parsed: {:?}", parsed);

  Ok(())
}