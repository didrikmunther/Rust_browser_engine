use std::cmp::PartialEq;

use AttrMap;
use HashMap;
use super::{Error, helper::{PreLexed, pre_lex}};

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
enum Lexed {
  Content(String),
  String(String),
  Token(Token),
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
  LT, // <
  GT, // >
  Equals, // =
  Slash, // /

  // 
  CloseTag, // </
  SelfClose // />
}

#[derive(Debug)]
pub struct Tag {
  pub name: String,
  pub attrs: AttrMap,
  pub status: TagStatus
}

#[derive(Debug, PartialEq)]
pub enum TagStatus {
  None, // temp
  Open, // <div>
  Close, // </div>
  SelfClose // <div />
}

#[derive(Debug)]
pub enum TagContents {
  Tag(Tag),
  Content(String)
}

fn get_tokens() -> HashMap<&'static str, Token> {
  map!{
    "<" => Token::LT,
    ">" => Token::GT,
    "=" => Token::Equals,
    "/" => Token::Slash
  }
}

fn tokenize(pre_lexed: Vec<PreLexed>) -> Result<Vec<Lexed>, Error> {
  let token_lookup = get_tokens();

  let is_token = |val: &str| -> Option<Token> {
    match token_lookup.get(val) {
      Some(token) => Some(token.clone()),
      None => None
    }
  };

  // let is_identifier = |val: &str| -> Option<String> {
  //   if val.len() <= 0 { return None; }

  //   for i in val.chars() {
  //     match i {
  //       'a' ... 'z' | 'A' ... 'Z' | '_' | '0' ... '9' => { },
  //       _ => { return None; }
  //     }
  //   }

  //   Some(String::from(val))
  // };

  let tokens = get_tokens();

  let is_string = |val: &str| -> Option<String> {
    let len = val.len();
    if len <= 0 { return None; }

    for (i, v) in tokens.iter() {
      if val.contains(i) {
        return None;
      }
    }

    // for (i, c) in val.chars().enumerate() {
    //   match c {
    //     // 'a' ... 'z' | 'A' ... 'Z' | '_' | '0' ... '9' | '\n' | '\t' | ' ' => { },
    //     '<' | '>' | '=' => return None,
    //     _ => { }
    //   }
    //   if i + 1 <= len {
    //     match (c, &val[i + 1]) {
    //       ('<', '/') | ('/', '>') => return None,
    //       _ => {}
    //     }
    //   }
    // }

    Some(String::from(val))
  };

  let mut tokenized: Vec<Lexed> = Vec::new();

  for (i, v) in pre_lexed.into_iter().enumerate() {
    match v {
      PreLexed::String(s) => {
        tokenized.push(Lexed::String(s))
      },
      PreLexed::Rest(ref rest) => {
        let length = rest.len();
        let mut pos = 0;
        let mut offset = length;

        // println!("offset: {:?}", &rest[pos..offset]);

        while offset >= 1 {
          if pos >= length || offset - pos <= 0 {
            // println!("breaking: pos: {}, offset: {}, length: {}", pos, offset, length);
            break;
          }
          let content = &rest[pos..offset];
          // println!("content: {:?}", content);

          offset -= 1;

          if let Some(token) = is_token(content) {
            tokenized.push(Lexed::Token(token));
          } else if let Some(s) = is_string(content) {
            tokenized.push(Lexed::Content(s));
          } else {
            continue;
          }

          // println!("tokenized: {:?}", tokenized);
          pos = offset + 1;
          offset = length;
          // if offset <= 0 {
          //   return Err(Error("Token not found".into_string()))
          // }
        }
      }
    }
  }

  Ok(tokenized)
}

fn compress_tokens(tokenized: Vec<Lexed>) -> Result<Vec<Lexed>, Error> {
  let tokens: HashMap<&str, Token> = map!{ // longest bottom
    "/>" => Token::SelfClose,
    "</" => Token::CloseTag
  };

  let single_tokens = get_tokens();

  let mut last_tokens: Vec<Token> = Vec::new();
  let mut result: Vec<Lexed> = Vec::new();

  let get_new_tokens = |last_tokens: &mut Vec<Token>| {
    let len = last_tokens.len();
    let mut new_tokens: Vec<Token> = Vec::new();
    
    if len > 1 {
      let mut pos = 0;
      let mut offset = len;

      loop {
        if pos > len || pos >= offset {
          break;
        }

        let current = &last_tokens[pos..offset];
        let mut string_current = String::new();
        for token in current.iter() {
          for (i, v) in single_tokens.iter() {
            if *v == *token {
              string_current.push_str(i.clone());
            }
          } 
        }

        println!("{:?}, {:?}", string_current, new_tokens);

        if string_current.len() == 1 {
          new_tokens.push(current[0].clone());
          pos += 1;
          offset = len;
          println!("{}, {}", pos, offset);
          continue;
        }

        for (&k, v) in tokens.iter() {
          if string_current == k {
            pos += k.len();
            offset = len;
            new_tokens.push(v.clone());
            continue;
          }
        }

        offset -= 1;
      }

    } else if len == 1 {
      new_tokens.push(last_tokens[0].clone());
    }

    new_tokens
  };

  for v in tokenized.into_iter() {
    match v {
      Lexed::Token(token) => {
        last_tokens.push(token);
      },
      Lexed::Content(_) | Lexed::String(_) => {
        for i in get_new_tokens(&mut last_tokens) {
          result.push(Lexed::Token(i));
        }
        last_tokens = Vec::new();
        result.push(v);
      }
    }
  }

  for i in get_new_tokens(&mut last_tokens) {
    result.push(Lexed::Token(i));
  }

  Ok(result)
}

fn taginize(tokenized: Vec<Lexed>) -> Result<Vec<TagContents>, Error> {
  let mut tags: Vec<TagContents> = Vec::new();

  let mut in_tag = false;
  let mut after_equals = false;
  let mut tag_status: TagStatus = TagStatus::None;

  let mut tag_name: Option<String> = None;
  let mut buf_attrs: Vec<(String, Option<String>)> = Vec::new();

  let mut pre_tag_buf: String = String::new();

  for (i, v) in tokenized.into_iter().enumerate() {
    match v {
      Lexed::String(s) => {
        if in_tag {
          if after_equals {
            after_equals = false;
            let last = buf_attrs.pop();
            match last {
              Some(last) => {
                let n = (last.0, Some(s.to_string()));
                buf_attrs.push(n);
              },
              None => return Err(Error("Wrong equals".to_string()))
            }
          }
        } else {
          pre_tag_buf.push_str(&s);
        }
      },
      Lexed::Content(s) => {
        if in_tag {
          if let None = tag_name {
            let splitted = s.split(" ");
            
            let mut first = true;

            for i in splitted.into_iter() {
              if first {
                first = false;
                tag_name = Some(i.to_string());
              } else {
                buf_attrs.push((i.to_string(), None));
              }
            }
          } else {
            let splitted = s.split(" ");
            for i in splitted.into_iter() {
              buf_attrs.push((i.to_string(), None));
            }
          }
        } else {
          pre_tag_buf.push_str(&s);
        }
      },
      Lexed::Token(token) => match token {
        Token::LT | Token::CloseTag => {
          if in_tag {
            return Err(Error("Tag error: <".to_string()));
          } else {
            if pre_tag_buf.len() > 0 {
              tags.push(TagContents::Content(pre_tag_buf));
              pre_tag_buf = String::new();
            }

            in_tag = true;
            tag_status = match token {
              Token::LT => TagStatus::Open,
              Token::CloseTag => TagStatus::Close,
              _ => TagStatus::None // will never happen
            }
          }
        },
        Token::GT | Token::SelfClose => {
          if !in_tag {
            return Err(Error((format!("Tag error: {:?}", token)).to_string()));
          } else {
            let name;
            if let Some(n) = tag_name {
              name = n;
            } else {
              return Err(Error("No tag name".to_string()));
            }

            let mut attrs: AttrMap = HashMap::new();
            for i in buf_attrs.into_iter() {
              if i.0.len() > 0 {
                attrs.insert(i.0, i.1);
              }
            }

            let tag = Tag {
              name,
              attrs,
              status: match token {
                Token::GT => tag_status,
                Token::SelfClose => {
                  if tag_status == TagStatus::Open {
                    TagStatus::SelfClose
                  } else {
                    return Err(Error("Cannot have self close and close on same tag".to_string()));
                  }
                },
                _ => TagStatus::None // will never happen
              }
            };

            tags.push(TagContents::Tag(tag));

            tag_name = None;
            buf_attrs = Vec::new();
            in_tag = false;
            after_equals = false;
            tag_status = TagStatus::None;
          }
        },
        Token::Equals => {
          if in_tag {
            after_equals = true;
          } else {
            pre_tag_buf.push('=');
          }
        },
        Token::Slash => {
          if in_tag {
            return Err(Error("Slash not allowed in tags".to_string()));
          } else {
            pre_tag_buf.push('/');
          }
        }
      }
    }
  }

  // Err(Error("what".to_string()))
  Ok(tags)
}

pub fn lex(query: String) -> Result<Vec<TagContents>, Error> {
  println!("[init lexer]");

  let pre_lexed = pre_lex(query)?;

  let tokenized = tokenize(pre_lexed)?;

  let tokenized = compress_tokens(tokenized)?;

  let taginized = taginize(tokenized)?;

  Ok(taginized)
}