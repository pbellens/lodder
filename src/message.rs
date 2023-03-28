use std::collections::HashMap;
use tui::style::Style;
use crate::vocab;
//use quick_xml::events::BytesStart;

#[derive(PartialEq)]
pub enum BareString {
    Owned(String),
    Vocab(vocab::Entry),
}

impl BareString {
    pub fn contains(&self, v: &vocab::Vocabulary, s: &str) -> bool {
        match self {
            BareString::Owned(str) => str.contains(s),
            BareString::Vocab(entry) => v.map(*entry).contains(s),
        }
    }

    pub fn as_ref<'a>(&'a self, v: &'a vocab::Vocabulary) -> &str {
        match self {
            BareString::Owned(str) => str,
            BareString::Vocab(entry) => v.map(*entry),
        }
    }
}

#[derive(PartialEq)]
pub struct PrettyString {
    pub string: BareString,
    pub style: Style,
}

impl PrettyString {
    pub fn bare(s: String) -> Self {
        PrettyString {
            string: BareString::Owned(s),
            style: Style::default(),
        }
    }
}

#[derive(PartialEq)]
pub struct MessageLine {
    pub key: PrettyString,
    pub value: PrettyString,
}

#[derive(PartialEq)]
pub enum Status {
    UNKNOWN,
    SUCCESS, 
    REJECTED,
}

#[derive(PartialEq)]
pub struct Message {
    //pub bs: BytesStart<'static>,
    pub contents: Vec<MessageLine>,
    pub stype: vocab::Entry,
}

#[derive(PartialEq)]
pub enum ParsedStart {
    Start(Overview),
    TryAgain,
    Eof,
}

#[derive(PartialEq)]
pub struct Overview {
    pub header: Vec<PrettyString>,
    pub msgs: Vec<Message>,
    pub stats: HashMap<String, f64>,
    pub status: Status,
    //txn_timestamp: String,
}

pub struct OverviewIterator<'a> {
    pub overview: &'a Overview,
    pub idx: usize,
}

impl<'a> Iterator for OverviewIterator<'a> {
    type Item = &'a PrettyString;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        self.overview.header.get(idx)
    }
}

impl<'a> IntoIterator for &'a Overview {
    type Item = &'a PrettyString;
    type IntoIter = OverviewIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        OverviewIterator {
            overview: self,
            idx: 0,
        }
    }
}
