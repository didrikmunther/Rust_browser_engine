use super::{Error, StyleNumber, lexer::{Lexed, Token}};

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
  Child,
  GeneralSibling,
  AdjacentSibling
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
pub struct Identifier {
  id: Vec<String>,
  class: Vec<String>,
  i_type: IdentifierType
}

#[derive(Debug)]
pub enum IdentifierType {
  Simple {
    tag_name: Option<String>
  },
  Everything, // *,
  Any
}

#[derive(Debug)]
pub struct Declaration {
  name: String,
  value: Value
}

#[derive(Debug)]
pub enum Value {
  Keyword(String),
  Length(StyleNumber, Unit),
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
  let comma_splitted = split(lexed, &Token::Comma)?;

  println!("comma_splitted: {:?}", comma_splitted);

  let mut selectors: Vec<Selector> = Vec::new();
  let mut selector_parser = SelectorParser::new();

  for i in comma_splitted.into_iter() {
    selector_parser.reset(i);
    selectors.push(selector_parser.parse_selector()?);
  }

  println!("selectors: {:?}", selectors);

  Ok(selectors)
}

fn split(lexed: Vec<Lexed>, with: &Token) -> Result<Vec<Vec<Lexed>>, Error> {
  let mut splitted: Vec<Vec<Lexed>> = Vec::new();
  let mut buf: Vec<Lexed> = Vec::new();

  for i in lexed.into_iter() {
    match i {
      Lexed::Token(ref token) => if token == with {
        splitted.push(buf);
        buf = Vec::new();
        continue;
      },
      _ => {}
    }
    buf.push(i);
  }

  splitted.push(buf);
  Ok(splitted)
}

struct SelectorParser {
  current: usize,
  lexed: Vec<Lexed>
}

impl SelectorParser {
  pub fn new() -> Self {
    Self {
      current: 0,
      lexed: Vec::new()
    }
  }

  pub fn reset(&mut self, lexed: Vec<Lexed>) {
    self.current = 0;
    self.lexed = lexed;
  }

  fn parse_selector(&mut self) -> Result<Selector, Error> {
    let mut selector = Selector {
      child: None,
      pseudo: Vec::new(),
      identifier: None
    };

    let mut first = true;
    let mut prev_token = &Token::None;

    let length = self.lexed.len();
    let self_point: *mut Self = self;

    loop {
      if self.current >= length {
        break;
      }

      let item = &self.lexed[self.current];

      match item {
        &Lexed::String(ref s) => return Err(Error(format!("Unexpected string in selector: {:?}", s))),
        &Lexed::Identifier(ref s) => {
          if first {
            selector.identifier = Some(Identifier {
              id: Vec::new(),
              class: Vec::new(),
              i_type: IdentifierType::Simple {
                tag_name: Some(s.to_string())
              }
            });
          } else {
            match prev_token {
              &Token::Hash | &Token::Dot => {
                // if let Some(identifier) = selector.identifier {
                  
                // } else {
                //   selector.identifier = Some(Identifier::Simple {
                //     tag_name: None,
                //     id: Vec::new(),
                //     class: Vec::new()
                //   });
                // }
              },
              &Token::Colon | &Token::DoubleColon => {

              },
              &Token::None | &Token::GT | &Token::Tilde | &Token::Plus => {
                selector.child = Some(Box::new(SelectorChild {
                  selector: unsafe {
                    (*self_point).parse_selector()?
                  },
                  child_type: match prev_token {
                    &Token::None => SelectorChildType::Descendant,
                    &Token::GT => SelectorChildType::Child,
                    &Token::Tilde => SelectorChildType::GeneralSibling,
                    &Token::Plus => SelectorChildType::AdjacentSibling,
                    _ => return Err(Error(format!("Unexpected token: {:?}", prev_token)))
                  }
                }))
              },
              _ => {
                return Err(Error(format!("Unexpected token in selector: '{:?}'", prev_token)))
              }
            }
          }

          prev_token = &Token::None;
        },
        &Lexed::Number(ref num) => {

        },
        &Lexed::Token(ref token) => {
          match token {
            &Token::Star => {
              if first {
                selector.identifier = Some(Identifier {
                  class: Vec::new(),
                  id: Vec::new(),
                  i_type: IdentifierType::Everything
                });
              } else {
                return Err(Error(format!("Unexpected '*' in selector")));
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

pub fn parse_declarations(lexed: Vec<Lexed>) -> Result<Vec<Declaration>, Error> {
  let splitted = split(lexed, &Token::SemiColon)?;

  println!("decls: {:?}", splitted);

  let mut declarations: Vec<Declaration> = Vec::new();

  for i in splitted.into_iter() {
    let length = i.len();
    if length <= 2 {
      continue; // forgiving errors
    }

    match &i[1] {
      &Lexed::Token(ref token) => match token {
        &Token::Colon => {},
        _ => continue
      }
      _ => continue
    }

    let name = match &i[0] {
      &Lexed::Identifier(ref identifier) => identifier.clone(),
      _ => continue
    };

    let value = match match &name as &str {
      "width" => parse_length,
      "display" => parse_keyword,
      _ => continue
    }(&i[2..]) {
      Ok(val) => val,
      Err(_) => continue
    };

    declarations.push(Declaration {
      name,
      value
    });
  }

  Ok(declarations)
}

fn parse_keyword(lexed: &[Lexed]) -> Result<Value, Error> {
  let length = lexed.len();

  if length <= 0 {
    return Err(Error(format!("Length too small for parse_keyword")));
  }

  let keyword = match &lexed[0] {
    &Lexed::Identifier(ref identifier) => identifier.clone(),
    _ => return Err(Error(format!("Not a keyword for parse_keyword")))
  };

  Ok(Value::Keyword(keyword))
}

fn parse_length(lexed: &[Lexed]) -> Result<Value, Error> {
  let length = lexed.len();

  if length <= 1 {
    return Err(Error(format!("Length too small for parse_length")));
  }

  let number = match &lexed[0] {
    &Lexed::Number(num) => {
      num
    },
    _ => return Err(Error(format!("Length not a number for parse_length")))
  };

  let unit = match &lexed[1] {
    &Lexed::Token(ref token) => match token {
      &Token::Px => Unit::Px,
      _ => return Err(Error(format!("Unknown unit for parse_length")))
    },
    _ => return Err(Error(format!("Unknown unit for parse_length")))
  };

  Ok(Value::Length(number, unit))
}