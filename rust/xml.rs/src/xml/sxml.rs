
use std::iter::Iterator;
use std::clone::Clone;
use std::io::Buffer;

pub enum Event {
    StartTag,
    EndTag,
    Character,
    CData,
    Instruction,
    DocType,
    Comment
}

enum State {
  State_Start,
  State_StartTag
}

struct Error {
    line: uint,
    col: uint,
    msg: ~str
}

pub struct Parser<T> {
  priv ch: char,
  priv reader: ~T,
  priv line: uint,
  priv col: uint,
}

impl<T: Iterator<char>> Parser<T> {

    fn new() -> Parser<T> {
        let parser = Parser {
            ch: -1,
            line: 0,
            col: 0,
            reader: ~T,
        }
        parser.bump();
        parser
    }
}

impl Parser<Buffer> {

    fn read_char(&mut self) -> IoResult<char> {
        let r = reader.read_char();
        match r {
            Ok(ch) => {
                self.ch = ch;
                match ch {
                    '\n' => {
                        self.line++;
                        self.col = 0;
                    }
                    _ => self.col++;
                }
            }
            _ =>
        }
        r
    }

    fn parse_whitespace(&mut self) {
        let mut iter = self.reader.chars
    }


    fn parse(&mut self) {
        let mut iter = self.reader.chars().peekable();

        iter.by_ref().take_while(|ch| ch.is_whitespace());

        let next = iter.peek();
        match next {
            Some(ch) => println!(ch)
            None => println!("EOF")
 
        }

    
    }

}

//impl<'a> Clone for Parser<'a> {
//  fn clone(&self) -> Parser<'a> {
//      Parser { buffer: self.buffer, bufferpos: self.bufferpos, ntokens: self.ntokens, taglevel: self.taglevel }
//  }
//}

impl Iterator<Token> for Parser {

  fn next(&mut self) -> Option<Token> {

      while !self.eof() {
            self.parse_whitespace();
            debug!("line:%u, col:%u", self.line, self.col);

            match self.ch {
                '<'
            }
      
      while !temp.root_found() {
          let start = temp.buffer.slice_from(temp.bufferpos);
          let lt = temp.buffer.trim_left();

          
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


