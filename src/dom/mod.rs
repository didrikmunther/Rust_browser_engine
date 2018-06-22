mod lexer;
mod parser;

pub struct Error(String);

pub fn parse_dom(query: String) -> Result<(), Error> {
  let lexed = lexer::lex(query)?;

  println!("{:?}", lexed);

  let parsed = parser::parse(lexed)?;

  println!("{:?}", parsed);

  Ok(())
}