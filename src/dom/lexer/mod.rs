use AttrMap;
use HashMap;

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
pub struct LexerErr(String);

#[derive(Debug)]
enum PreLexed {
  String(String),
  Rest(String)
}

#[derive(Debug)]
enum Lexed {
  Content(String),
  String(String),
  Token(Token),
}

#[derive(Debug, Clone)]
enum Token {
  LT, // <
  GT, // >
  Equals, // =
  CloseTag, // </
  SelfClose // />
}

#[derive(Debug)]
pub struct Tag {
  name: String,
  attrs: AttrMap,
  status: TagStatus
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

fn pre_lex(query: String) -> Result<Vec<PreLexed>, LexerErr> {
  let mut result: Vec<PreLexed> = Vec::new();
  let mut buf: String = String::new();
  let mut is_string = false;

  for (i, c) in query.chars().enumerate() {
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

fn tokenize(pre_lexed: Vec<PreLexed>) -> Result<Vec<Lexed>, LexerErr> {
  let token_lookup: HashMap<&str, Token> = map!{
    "<" => Token::LT,
    ">" => Token::GT,
    "=" => Token::Equals,
    "</" => Token::CloseTag,
    "/>" => Token::SelfClose
  };

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

  let is_string = |val: &str| -> Option<String> {
    if val.len() <= 0 { return None; }

    for i in val.chars() {
      match i {
        'a' ... 'z' | 'A' ... 'Z' | '_' | '0' ... '9' | '\n' | '\t' | ' ' => { },
        _ => { return None; }
      }
    }

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
          //   return Err(LexerErr("Token not found".into_string()))
          // }
        }
      }
    }
  }

  Ok(tokenized)
}

fn taginize(tokenized: Vec<Lexed>) -> Result<Vec<TagContents>, LexerErr> {
  let mut tags: Vec<TagContents> = Vec::new();

  let mut in_tag = false;
  let mut after_equals = false;
  let mut tag_status: TagStatus = TagStatus::None;

  let mut tag_name: Option<String> = None;
  let mut buf_attrs: Vec<(String, String)> = Vec::new();

  for (i, v) in tokenized.into_iter().enumerate() {
    match v {
      Lexed::String(s) => {
        if after_equals {
          after_equals = false;
          let last = buf_attrs.pop();
          match last {
            Some(last) => {
              let n = (last.0, s.to_string());
              buf_attrs.push(n);
            },
            None => return Err(LexerErr("Wrong equals".to_string()))
          }
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
                buf_attrs.push((i.to_string(), "".to_string()));
              }
            }
          } else {
            let splitted = s.split(" ");
            for i in splitted.into_iter() {
              buf_attrs.push((i.to_string(), "".to_string()));
            }
          }
        }
      },
      Lexed::Token(token) => match token {
        Token::LT | Token::CloseTag => {
          if in_tag {
            return Err(LexerErr("Tag error: <".to_string()));
          } else {
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
            return Err(LexerErr((format!("Tag error: {:?}", token)).to_string()));
          } else {
            let name;
            if let Some(n) = tag_name {
              name = n;
            } else {
              return Err(LexerErr("No tag name".to_string()));
            }

            let mut attrs: AttrMap = HashMap::new();
            for i in buf_attrs.into_iter() {
              attrs.insert(i.0, i.1);
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
                    return Err(LexerErr("Cannot have self close and close on same tag".to_string()));
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
          }
        }
      }
    }
  }

  // Err(LexerErr("what".to_string()))
  Ok(tags)
}

pub fn lex(query: String) -> Result<Vec<TagContents>, LexerErr> {
  println!("[init lexer]");

  let pre_lexed = pre_lex(query)?;

  println!("{:?}", pre_lexed);

  let tokenized = tokenize(pre_lexed)?;

  println!("{:?}", tokenized);

  let taginized = taginize(tokenized)?;

  println!("{:?}", taginized);

  Err(LexerErr("test".to_string()))
}