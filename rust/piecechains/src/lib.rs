use std::fmt;
use std::vec::Vec;
use std::string::String;
use std::collections::dlist::DList;
use std::path::BytesContainer;

pub struct Span<'a> {
    offset: usize,
    len: usize,
    buffer: &'a [u8]
}

impl<'a> fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = String::from_utf8_lossy(self.buffer);
        write!(f, "{}", text)
    }
}

struct SpanRange<'a> {
    first: &'a Span<'a>,
    last:  &'a Span<'a>,
    boundary: bool,
    sequence_length: usize,
}

pub struct Sequence<'a> {
    span_list: DList<Span<'a>>,
    // this is the modifications buffer
    buffer: Vec<u8>,
}

impl<'a> Sequence<'a> {

    fn span_for_index(&'a self, index: usize) -> Option<&'a Span> {
        let mut curr_len = 0;
        for span in self.span_list.iter() {
            if (index >= curr_len && index < curr_len + span.len) {
                return Some(span);
            }
            curr_len = curr_len + span.len;
        }
        return None;
    }

    pub fn new() -> Sequence<'a> {
        let list: DList<Span> = DList::new();
        let buffer: Vec<u8> = Vec::new();
        Sequence{span_list: list, buffer: buffer}
    }

    pub fn new_from_str(text: &'a str) -> Sequence<'a> {
        let mut seq = Sequence::new();
        let initial_span = Span{offset: 0, len: text.len(), buffer: text.as_bytes()};
        seq.span_list.push_back(initial_span);
        return seq;
    }

    pub fn insert(&mut self, index: usize, buffer: &str, len: usize) {
        
    }

}


#[test]
fn test_creation_from_string() {
    let seq = Sequence::new_from_str("The Quick Brown");
}

#[test]
fn test_creation_empty() {

    let seq = Sequence::new();
}
