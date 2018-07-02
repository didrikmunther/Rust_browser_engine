use std::collections::HashMap;
use super::{Error, helper::{PreLexed, pre_lex}};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  None,

  GT,
  BraceOpen,
  BraceClose,
  Comma,
  Dot,
  ParOpen,
  ParClose,
  SemiColon,
  Colon,
  DoubleColon,
  Hash,
  Star,
  Tilde,
  Plus,

  RGB,
  Px,
  Percent
}

pub type StyleNumber = f32;

#[derive(Debug, Clone)]
pub enum Lexed {
  Identifier(String),
  Number(StyleNumber),
  String(String),
  Token(Token),
}

fn get_tokens() -> HashMap<&'static str, Token> {
  map!{
    ">" => Token::GT,
    "{" => Token::BraceOpen,
    "}" => Token::BraceClose,
    ":" => Token::Colon,
    "::" => Token::DoubleColon,
    ";" => Token::SemiColon,
    "#" => Token::Hash,
    "," => Token::Comma,
    "." => Token::Dot,
    "*" => Token::Star,
    "~" => Token::Tilde,
    "+" => Token::Plus,
    "px" => Token::Px,
    "%" => Token::Percent
  }
}

fn tokenize(pre_lexed: Vec<PreLexed>) -> Result<Vec<Lexed>, Error> {
  let mut result: Vec<Lexed> = Vec::new();
  let token_lookup = get_tokens();

  let is_identifier = |query: &str| -> Option<String> {
    if query.len() <= 0 { return None; }
    let mut expect_more = false; // cannot be named just '_', '-' or '0'...'9'
    let mut first = true;

    for i in query.chars() {
      match i {
        '0' ... '9' => {
          if first {
            return None;
          } else {
            expect_more = false;
          }
        }
        'a' ... 'z' | 'A' ... 'Z' => expect_more = false,
        '_' | '-' => expect_more = first,
        _ => return None
      }
      first = false;
    }

    if expect_more {
      return None;
    }

    Some(String::from(query))
  };

  //std::num::ParseFloatError

  let is_number = |query: &str| -> Option<StyleNumber> {
    match query.parse::<StyleNumber>() {
      Ok(num) => Some(num),
      _ => None
    }
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
          } else if let Some(num) = is_number(content) {
            result.push(Lexed::Number(num));
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