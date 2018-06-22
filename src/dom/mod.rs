mod lexer;
mod parser;

pub fn parse_dom(query: String) {
  println!("{:?}", lexer::lex(query));
}