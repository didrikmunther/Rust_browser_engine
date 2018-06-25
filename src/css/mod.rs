use super::helper;
use helper::Error;

mod lexer;
mod parser;

pub fn parse_css(query: String) {
  println!("[init css]");

  println!("lexed: {:?}", lexer::lex(query));
}