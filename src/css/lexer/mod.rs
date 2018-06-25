use std::collections::HashMap;
use super::{Error, helper::{PreLexed, pre_lex}};

#[derive(Debug, Clone)]
pub enum Token {
  GT,
  BraceOpen,
  BraceClose,
  Comma,
  ParOpen,
  ParClose,
  SemiColon,
  Colon,
  Hash,
  Star,

  RGB
}

#[derive(Debug)]
pub enum Lexed {
  Identifier(String),
  String(String),
  Token(Token),
}

// enum Selector {
//   Simple {
//     ids: Vec<String>,
//     classes: Vec<String>,
//     tag_names: Vec<String>
//   }
// }

// struct Declaration {
//   name: String,
//   value: Value
// }

// struct Value {
  
// }

// struct Rule {

// }

// struct Stylesheet {
//   rules: Vec<Rule>
// }

fn get_tokens() -> HashMap<&'static str, Token> {
  map!{
    ">" => Token::GT,
    "{" => Token::BraceOpen,
    "}" => Token::BraceClose,
    ":" => Token::Colon,
    ";" => Token::SemiColon,
    "#" => Token::Hash
  }
}

// fn trim(query: &str) -> String {
//   query.trim()
// }

fn tokenize(pre_lexed: Vec<PreLexed>) -> Result<Vec<Lexed>, Error> {
  let mut result: Vec<Lexed> = Vec::new();
  let token_lookup = get_tokens();

  let is_identifier = |query: &str| -> Option<String> {
    if query.len() <= 0 { return None; }
    let mut expect_more = false; // cannot be named just '_', '-' or '0'...'9'
    let mut first = true;

    for i in query.chars() {
      match i {
        'a' ... 'z' | 'A' ... 'Z' => expect_more = false,
        '_' | '-' | '0' ... '9' => expect_more = first,
        _ => { return None; }
      }
      first = false;
    }

    if expect_more {
      return None;
    }

    Some(String::from(query))
  };

  let is_token = |query: &str| -> Option<Token> {
    match token_lookup.get(query) {
      Some(token) => Some(token.clone()),
      None => None
    }
  };

  for i in pre_lexed.into_iter() {
    match i {
      PreLexed::String(s) => result.push(Lexed::String(s)),
      PreLexed::Rest(rest) => {
        let length = rest.len();
        let mut pos = 0;
        let mut offset = length;

        while offset >= 1 {
          if pos >= length {
            // println!("breaking: pos: {}, offset: {}, length: {}", pos, offset, length);
            break;
          }

          if offset - pos <= 0 {
            return Err(Error(format!("Token not recognized: \'{}\'", &rest[pos..offset+1])))
          }

          let content = &rest[pos..offset].trim();
          // println!("content: {:?}", content);

          offset -= 1;

          if let Some(token) = is_token(content) {
            result.push(Lexed::Token(token));
          } else if let Some(identifier) = is_identifier(content) {
            result.push(Lexed::Identifier(identifier));
          } else {
            continue;
          }

          // println!("tokenized: {:?}", tokenized);
          pos = offset + 1;
          offset = length;
          // println!("{}, {}, {}", pos, offset, length);
          // if offset == pos {
          //   return Err(Error("Token not found".to_string()))
          // }
        }
      }
    }
  }

  Ok(result)
}

pub fn lex(query: String) -> Result<Vec<Lexed>, Error> {
  let pre_lexed = pre_lex(query)?;

  let tokenized = tokenize(pre_lexed)?;

  Ok(tokenized)
}