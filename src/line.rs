use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::message;
use crate::parse;
use crate::stats;
use crate::vocab::Vocab;

pub struct QuickIt {
    reader: Reader<BufReader<File>>,
    buf: Vec<u8>,
    idx: usize,
    v: Arc<Mutex<Vocab>>,
    stats: Vec<stats::StatsCfg>,
}

impl QuickIt {
    pub fn new(f: PathBuf, v: Arc<Mutex<Vocab>>, stats: Vec<stats::StatsCfg>) -> Self {
        QuickIt {
            reader: Reader::from_file(f).expect("Could not create XML reader"),
            buf: vec![],
            idx: 0,
            v,
           stats
        }
    }

    fn step(&mut self, idx: usize) -> message::ParsedStart {
        let statls = self.stats
            .iter().map(|s| {
                |k: &[u8], v: &[u8]| {  
                    if k == s.id.as_bytes() {
                        Some(std::str::from_utf8(v).unwrap().to_owned().parse::<f64>().expect("not a number"))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<_>>();
        match self.reader.read_event_into(&mut self.buf) {
            Err(e) => panic!("Error at position {}: {:?}", self.reader.buffer_position(), e),
            Ok(Event::Eof) => message::ParsedStart::Eof,
            Ok(Event::Start(e)) if e.name().as_ref() == b"MultiMessage" => {
                parse::parse(
                    &mut self.reader, 
                    &self.v,
                    e, 
                    idx, 
                    statls.as_ref())
            }
            _ => message::ParsedStart::TryAgain,
        }
    }
}

impl Iterator for QuickIt {
    type Item = message::Overview;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = self.step(self.idx);
        while v == message::ParsedStart::TryAgain {
            v = self.step(self.idx);
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        self.buf.clear();

        match v {
            message::ParsedStart::Start(ss) => { self.idx += 1; Some(ss) }, 
            message::ParsedStart::TryAgain => unreachable!(),
            message::ParsedStart::Eof => None,
        }
    }
}
