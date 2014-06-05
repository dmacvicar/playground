
use std::iter::Iterator;
use std::clone::Clone;

pub enum Token {
    StartTag,
    EndTag,
    Character,
    CData,
    Instruction,
    DocType,
    Comment,
    ErrorXmlInvalid,
    Success,
    ErrorBufferDry,
    ErrorTokensFull,
}

// constants.rs
// static LANGUAGE: &'static str = "Rust";
// static THRESHOLD: int = 10;
static TAG_MINSIZE: uint = 3;

pub struct Parser<'a> {
    buffer: &'a str,
    /* Current offset into buffer - all XML data before this position has been successfully parsed */
    bufferpos: uint,
    /* Number of tokens filled with valid data by the parser */
    ntokens: uint,
    /* Used internally - keeps track of number of unclosed XML elements to detect start and end of document */
    taglevel: uint,
}

impl<'a> Parser<'a> {

    pub fn new(buffer: &'a str) -> Parser<'a> {
        Parser {buffer: buffer, bufferpos: 0, ntokens: 0, taglevel: 0}
    }

    fn root_found(&'a self) -> bool {
        self.taglevel > 0
    }

    fn root_parsed(&'a self) -> bool {
        self.taglevel == 0
    }

    fn parse() -> Token {
        ErrorBufferDry
    }
}

impl<'a> Clone for Parser<'a> {
  fn clone(&self) -> Parser<'a> {
      Parser { buffer: self.buffer, bufferpos: self.bufferpos, ntokens: self.ntokens, taglevel: self.taglevel }
  }
}

impl<'a> Iterator<Token> for Parser<'a> {

  fn next(&mut self) -> Option<Token> {

      let mut temp = self.clone();

      while !self.root_found() {
          let start = self.buffer.slice_from(self.bufferpos);
          let lt = self.buffer.trim_left();
          
      }

     // Some(ErrorBufferDry)
     None
  }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_parser() {
        let xmldata = r##"<test attr1="hello">some text</test>"##;
        let mut parser = super::Parser::new(xmldata);
        for i in parser {
            println!("Boom!");
        }
    }
}


