use std::collections::HashMap;
use quick_xml::events::BytesStart;

#[derive(PartialEq)]
pub enum Format {
    Bold, 
    UnderLine,
    Normal,
}

#[derive(PartialEq)]
pub struct FormattedString {
    pub string: String, 
    pub format: Format,
}

#[derive(PartialEq)]
pub struct Message {
    pub bs: BytesStart<'static>,
    pub stype: usize,
    //pub sid: String,
    //pub msgid: String,
    //pub decid: String,
    //pub length: String,
    pub contents: Vec<FormattedString>
}

#[derive(PartialEq)]
pub enum ParsedStart {
    Start(Overview),
    TryAgain,
    Eof,
}

#[derive(PartialEq)]
pub struct Overview {
    pub frame_id: String,
    pub date: String,
    pub time: String,
    pub size: String,
    pub no: String,
    pub msgc: String,
    pub product_id: String,
    pub txn: String,
    pub msgs: Vec<Message>,
    pub stats: HashMap<String, f64>,
    //txn_timestamp: String,
}

pub struct OverviewIterator<'a> {
    pub overview: &'a Overview,
    pub idx: usize,
}

impl<'a> Iterator for OverviewIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        match idx {
            0 => Some(&self.overview.frame_id),
            1 => Some(&self.overview.date),
            2 => Some(&self.overview.time),
            3 => Some(&self.overview.size),
            4 => Some(&self.overview.no),
            5 => Some(&self.overview.msgc),
            6 => Some(&self.overview.product_id),
            7 => Some(&self.overview.txn),
            _ => None,
        }
    }
}

impl<'a> IntoIterator for &'a Overview {
    type Item = &'a str;
    type IntoIter = OverviewIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        OverviewIterator {
            overview: self,
            idx: 0
        }
    }
}

