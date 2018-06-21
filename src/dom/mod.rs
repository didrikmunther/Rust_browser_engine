mod lexer;
mod parser;

pub fn parse_dom(query: String) {
  lexer::lex(query);
}