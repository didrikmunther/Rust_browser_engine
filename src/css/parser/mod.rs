use super::{Error, lexer::{Lexed, Token}};

#[derive(Debug)]
pub struct Stylesheet {
  rules: Vec<Rule>
}

#[derive(Debug)]
pub struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>
}

#[derive(Debug)]
pub struct Selector {
  identifier: Option<Identifier>,
  pseudo: Vec<Pseudo>,
  child: Option<Box<SelectorChild>>
}

#[derive(Debug)]
pub struct SelectorChild {
  selector: Selector,
  child_type: SelectorChildType
}

#[derive(Debug)]
pub enum SelectorChildType {
  Descendant,
  Combinator,
  Sibling,
}

#[derive(Debug)]
pub enum Pseudo {
  Match(PseudoType),
  Not(PseudoType)
}

#[derive(Debug)]
pub enum PseudoType {
  FirstChild,
  LastChild
}

#[derive(Debug)]
pub enum Identifier {
  Simple {
    tag_name: Option<String>,
    id: Vec<String>,
    class: Vec<String>
  },
  Everything // *
}

#[derive(Debug)]
pub struct Declaration {
  name: String,
  value: Value
}

#[derive(Debug)]
pub enum Value {
  Keyword(String),
  Length(f32, Unit),
  Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
  }
}

#[derive(Debug)]
pub enum Unit {
  Px
}

pub fn parse_stylesheet(lexed: Vec<Lexed>) -> Result<Stylesheet, Error> {
  let length = lexed.len();

  let mut splitted: Vec<(Vec<Lexed>, Vec<Lexed>)> = Vec::new();
  let mut buf: Vec<Lexed> = Vec::new();

  enum BufStatus {
    Selector,
    Declaration
  }

  let status = BufStatus::Selector;

  for i in lexed.into_iter() {
    match i {
      Lexed::Token(ref token) => match token {
        &Token::BraceOpen => {
          splitted.push((buf, Vec::new()));
          buf = Vec::new();
          let status = BufStatus::Declaration;
          continue;
        },
        &Token::BraceClose => {
          let mut last = splitted.pop().unwrap();
          last.1 = buf;
          splitted.push(last);
          buf = Vec::new();
          let status = BufStatus::Selector;
          continue;
        },
        _ => {}
      },
      _ => {}
    }
    buf.push(i);
  }

  // println!("here: {:?}", splitted);

  let mut sheet = Stylesheet {
    rules: Vec::new()
  };

  for i in splitted.into_iter() {
    let selectors = parse_selectors(i.0)?;
    let declarations = parse_declarations(i.1)?;

    sheet.rules.push(Rule {
      selectors,
      declarations
    });
  }

  Ok(sheet)
}

fn parse_selectors(lexed: Vec<Lexed>) -> Result<Vec<Selector>, Error> {
  let mut comma_splitted: Vec<Vec<Lexed>> = Vec::new();
  let mut buf: Vec<Lexed> = Vec::new();

  for i in lexed.into_iter() {
    match i {
      Lexed::Token(ref token) => match token {
        &Token::Comma => {
          comma_splitted.push(buf);
          buf = Vec::new();
          continue;
        },
        _ => {}
      },
      _ => {}
    }
    buf.push(i);
  }

  comma_splitted.push(buf);

  let mut selectors: Vec<Selector> = Vec::new();
  let mut selector_parser = SelectorParser {
    current: 0,
    lexed: Vec::new()
  };

  for i in comma_splitted.into_iter() {
    selector_parser.current = 0;
    selector_parser.lexed = i;

    selectors.push(selector_parser.parse_selector()?);
  }

  println!("selectors: {:?}", selectors);

  Ok(selectors)
}

struct SelectorParser {
  current: usize,
  lexed: Vec<Lexed>
}

impl SelectorParser {
  fn parse_selector(&mut self) -> Result<Selector, Error> {
    let mut selector = Selector {
      child: None,
      pseudo: Vec::new(),
      identifier: None
    };

    let mut first = true;
    let mut prev_token = &Token::None;

    let length = self.lexed.len();
    let self_point: *mut Self = unsafe {
      self
    };

    loop {
      if self.current >= length {
        break;
      }

      let item = &self.lexed[self.current];

      match item {
        &Lexed::String(ref s) => return Err(Error(format!("Unexpected string in selector: {:?}", s))),
        &Lexed::Identifier(ref s) => {
          if first {
            selector.identifier = Some(Identifier::Simple {
              tag_name: Some(s.to_string()),
              id: Vec::new(),
              class: Vec::new()
            });
          } else if prev_token == &Token::Hash {

          } else if prev_token == &Token::Dot {

          } else {
            selector.child = Some(Box::new(SelectorChild {
              selector: unsafe {
                (*self_point).parse_selector()?
              },
              child_type: match prev_token {
                &Token::None => SelectorChildType::Combinator,
                &Token::GT => SelectorChildType::Descendant,
                _ => return Err(Error(format!("Unexpected token: {:?}", prev_token)))
              }
            }))
          }

          prev_token = &Token::None;
        },
        &Lexed::Token(ref token) => {
          match token {
            &Token::Star => {
              if first {

              }
            },
            _ => {}
          }
          prev_token = token;
        }
      }

      first = false;
      self.current += 1;
    }

    Ok(selector)
  }
}

// fn parse_selector(mut lexed: Vec<Lexed>) -> Result<Selector, Error> {
//   // let a = lexed.pop_front().unwrap();

//   Err(Error("no".to_string()))

//   // Ok(Selector {
//   //   child: None,
//   //   pseudo: Vec::new(),
//   //   identifier: Identifier::Simple {
//   //     class: Vec::new(),
//   //     id: Vec::new(),
//   //     tag_name: Some("head".to_string())
//   //   }
//   // })
// }

pub fn parse_declarations(lexed: Vec<Lexed>) -> Result<Vec<Declaration>, Error> {


  Ok(Vec::new())
}