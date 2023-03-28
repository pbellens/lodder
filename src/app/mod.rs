use crate::{message::{self, Overview}, parse, stats::{self, HeaderElement}, ui, vocab::{self, Vocabulary}};
use std::{
    io,
    path::PathBuf,
    sync::mpsc::Sender,
    time::Duration,
};
use tui::{
    backend::Backend,
    widgets::{ListState, TableState},
    Terminal,
};
mod events;
pub mod grow;

#[derive(PartialEq)]
pub enum Focus {
    MMOVERVIEW,
    MOVERVIEW,
    MDETAILS,
}

#[derive(PartialEq)]
pub enum StatStyle {
    NOTHING,
    LINES,
    TABLES,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ExCommand {
    Jump,
    Search,
}

#[derive(PartialEq)]
pub enum Mode {
    View,
    Help,
    Ex(ExCommand),
    Finding/*(String)*/,
}

#[derive(Eq, PartialEq)]
pub enum ParseStatus {
    Waiting,
    Parsing(usize),
    Done,
}

pub struct App {
    pub items: Vec<message::Overview>,
    pub state: TableState,
    pub selected: Option<usize>,
    pub skip: usize,
    pub window: Option<(usize, usize)>,
    pub focus: Focus,
    pub mstate: grow::GrowingListState<ListState>,
    pub lstate: grow::GrowingListState<TableState>,
    pub parsing: ParseStatus,
    pub mode: Mode,
    pub statstyle: StatStyle,
    pub command: String,
}

pub enum ParseCommand {
    Parse(usize),
    Stop,
}
pub enum Parsed {
    Chunk{messages: Vec<message::Overview>, vocabulary: Vocabulary},
    Done,
}

impl App {
    pub fn new(_lazy: bool) -> App {
        App {
            items: vec![],
            state: TableState::default(),
            selected: None,
            skip: 0,
            window: None,
            focus: Focus::MMOVERVIEW,
            mstate: grow::GrowingListState::default(),
            lstate: grow::GrowingListState::default(),
            parsing: ParseStatus::Waiting,
            mode: Mode::View,
            statstyle: StatStyle::LINES,
            command: "".to_owned(),
        }
    }

    pub fn target(&mut self, tx: &Sender<ParseCommand>, target: usize) {
        if target >= self.items.len() {
            if ParseStatus::Waiting == self.parsing {
                let req = target - self.items.len() + 1;
                self.parsing = ParseStatus::Parsing(req);
                tx.send(ParseCommand::Parse(req)).unwrap();
            }
        } else {
            self.selected = Some(target);
        }
    }

    pub fn find<'a, I>(&'a self, v: &vocab::Vocabulary, items: I, target: &str, skip: usize) -> Option<usize>
    where
        I: Iterator<Item = (usize, &'a Overview)>,
    {
        items
            .skip(skip)
            .find(|(_i, item)| item.header.iter().any(|s| s.string.contains(v, target)))
            .map(|(i, _item)| i)
    }

    pub fn next(&mut self, tx: &Sender<ParseCommand>, jump: usize) {
        match self.focus {
            Focus::MMOVERVIEW => {
                self.target(
                    tx,
                    match self.selected {
                        Some(c) => c + jump,
                        None => jump - 1,
                    },
                );
            }
            Focus::MOVERVIEW => self.mstate.next(jump),
            Focus::MDETAILS => self.lstate.next(jump),
        }
    }

    pub fn previous(&mut self, jump: usize) {
        match self.focus {
            Focus::MMOVERVIEW => {
                let target = match self.selected {
                    Some(i) => i.saturating_sub(jump),
                    None => 0,
                };
                self.selected = Some(target);
            }
            Focus::MOVERVIEW => self.mstate.prev(jump),
            Focus::MDETAILS => self.lstate.prev(jump),
        };
    }
}

#[allow(unreachable_code)]
pub fn run_app<B: Backend>(
    path: PathBuf,
    terminal: &mut Terminal<B>,
    mut app: App,
    statfils: &[stats::StatsCfg],
    header: &[HeaderElement],
) -> io::Result<()> {
    let (ctx, crx) = std::sync::mpsc::channel::<ParseCommand>();
    let (ktx, prx) = std::sync::mpsc::channel::<Parsed>();
    let blafils = statfils.to_vec();
    let _handle = std::thread::spawn(move || {
        let mut qit = parse::line::QuickIt::new(path, blafils);
        loop {
            if let Ok(cmd) = crx.recv() {
                let snapshot = qit.v.snapshot();
                match cmd {
                    ParseCommand::Parse(mut l) => {
                        while l > 0 {
                            let mut parsed: Vec<message::Overview> = vec![];
                            let mut b = 100;
                            while let Some(chunk) = qit.next() {
                                parsed.push(chunk);
                                l -= 1;
                                b -= 1;
                                if l == 0 || b == 0 {
                                    break;
                                }
                            }
                            if parsed.is_empty() {
                                ktx.send(Parsed::Done).unwrap();
                                break;
                            } else {
                                ktx.send(
                                    Parsed::Chunk{
                                        messages: parsed, 
                                        vocabulary: qit.v.snip(&snapshot)
                                    })
                                    .unwrap();
                            }
                        }
                    }
                    ParseCommand::Stop => break,
                }
            }
        }
    });

    let mut vocabulary = vocab::Vocabulary::new(3);

    loop {
        if app.parsing != ParseStatus::Done {
            if let ParseStatus::Parsing(target) = app.parsing {
                if let Ok(parsed) = prx.try_recv() {
                    match parsed {
                        Parsed::Chunk{messages: c, vocabulary: v} => {
                            if c.len() > 0 {
                                let left = target - c.len();
                                app.parsing = if left > 0 {
                                    ParseStatus::Parsing(left)
                                } else {
                                    ParseStatus::Waiting
                                };
                                app.items.extend(c);
                                vocabulary.join(v);
                                app.selected = Some(app.items.len() - 1);
                            }
                        }
                        Parsed::Done => {
                            app.parsing = ParseStatus::Done;
                        }
                    }
                };
            }
        }

        terminal.draw(|f| ui::ui(f, &mut app, &vocabulary, statfils, header))?;

        let duration = if app.parsing != ParseStatus::Done {
            Duration::from_millis(10)
        } else {
            Duration::from_secs(1)
        };
        match events::handle(&mut app, &vocabulary, duration, &ctx) {
            Ok(true) => { /* send parse cmds? */ }
            Ok(false) => break,
            Err(_) => todo!(),
        }
    }

    ctx.send(ParseCommand::Stop).unwrap();
    _handle.join().unwrap();

    log::info!("vocabulary: size {}", 
        vocabulary.words.iter().map(|ws| ws.len()).sum::<usize>());

    Ok(())
}
