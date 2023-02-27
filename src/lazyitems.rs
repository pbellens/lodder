//use std::{io, path::PathBuf, sync::{Arc, Mutex, mpsc::Sender}};
//use crate::{line, message, stats, ui, vocab};
//use crossterm::event::{self, Event, KeyCode, KeyModifiers};
//use tui::{
//    backend::Backend,
//    widgets::{TableState, ListState},
//    Terminal, 
//};
//
//pub struct LazyItems<'a> {
//    pub items: Vec<message::Overview>,
//    pub it: Option<line::QuickIt<'a>>,
//}
//
//impl<'a> LazyItems<'a> {
//    fn new(lit: line::QuickIt<'a>, lazy: bool) -> Self {
//        if lazy {
//            LazyItems {
//                items: vec![],
//                it: Some(lit),
//            }
//        } else {
//            LazyItems {
//                items: lit.collect(),
//                it: None,
//            }
//        }
//    }
//
//    fn next(&mut self, idx: Option<usize>) -> usize {
//        match &mut self.it {
//            Some(i) => {
//                if idx.is_none() {
//                    return match i.next() {
//                        Some(v) => { self.items.push(v); 0 }
//                        None => 0,
//                    }
//                }
//                let newidx = idx.unwrap() + 1;
//                if newidx >= self.items.len() {
//                    return match i.next() {
//                        Some(v) => { self.items.push(v); newidx }
//                        None => idx.unwrap(),
//                    }
//                }
//                newidx
//            },
//            None => {
//                match idx {
//                    Some(i) => {
//                        if i >= self.items.len() - 1 {
//                            i//0
//                        } else {
//                            i + 1
//                        }
//                    },
//                    None => 0,
//                }
//            }
//        }
//    }
//
//    fn fill(&mut self) {
//        let it = self.it.take();
//        match it {
//            Some(i) => {
//                self.items.extend(i);
//            },
//            None => (),
//        };
//    }
//}
