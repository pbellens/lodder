use std::{io, path::PathBuf, sync::{Arc, Mutex, mpsc::Sender}, time::Duration};
use crate::{line, message, stats, ui, vocab};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui::{
    backend::Backend,
    widgets::{TableState, ListState},
    Terminal, 
};

#[derive(PartialEq)]
pub enum Focus {
    MMOVERVIEW,
    MOVERVIEW,
    MDETAILS,
}

pub struct GrowingListState {
    pub state: ListState,
    pub length: Option<usize>,
}

impl GrowingListState {
    pub fn default() -> Self {
        GrowingListState {
            state: ListState::default(),
            length: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.length.unwrap() - 1 {
                    i//0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    i//self.items.len() - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[derive(Eq, PartialEq)]
pub enum ParseStatus {
    Waiting,
    Parsing(usize),
    Done
}

pub struct App {
    pub items: Vec<message::Overview>,
    pub state: TableState,
    pub selected: Option<usize>,
    pub skip: usize,
    pub window: Option<(usize, usize)>,
    pub focus: Focus,
    pub mstate: GrowingListState,
    pub lstate: GrowingListState,
    pub parsing: ParseStatus,
    pub help: bool,
}

pub enum ParseCommand {
    Parse(usize), 
    Stop
}
pub enum Parsed {
    Chunk(Vec<message::Overview>), 
    Done
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
            mstate: GrowingListState::default(),
            lstate: GrowingListState::default(),
            parsing: ParseStatus::Waiting,
            help: false,
        }
    }

    pub fn target(&mut self, tx: &Sender<ParseCommand>, target: usize) {
        match self.focus {
            Focus::MMOVERVIEW => {
                if target >= self.items.len() {
                    if ParseStatus::Waiting == self.parsing {
                        let req = target - self.items.len() + 1;
                        self.parsing = ParseStatus::Parsing(req);
                        tx.send(ParseCommand::Parse(req)).unwrap();
                    }
                }
                else {
                    self.selected = Some(target);
                }
            },
            Focus::MOVERVIEW => self.mstate.next(),
            Focus::MDETAILS => self.lstate.next(),
        };
    }

    pub fn next(&mut self, tx: &Sender<ParseCommand>, jump: usize) {
        self.target(
            tx, 
            match self.selected {
                Some(c) => c + jump,
                None => jump - 1,
            });
    }

    pub fn previous(&mut self, jump: usize) {
        match self.focus {
            Focus::MMOVERVIEW => {
                let target = match self.selected {
                    Some(i) => i.saturating_sub(jump),
                    None => 0,
                };
                self.selected = Some(target);
            },
            Focus::MOVERVIEW => self.mstate.prev(),
            Focus::MDETAILS => self.lstate.prev(),
        };
    }
}

#[allow(unreachable_code)]
pub fn run_app<B: Backend>(
    path: PathBuf,
    terminal: &mut Terminal<B>, 
    mut app: App, 
    statfils: &[stats::StatsCfg], 
    header: &[String], 
    vocab: Arc<Mutex<vocab::Vocab>>) -> io::Result<()> 
{

    let vocab2 = Arc::clone(&vocab);
    let (ctx, crx) = std::sync::mpsc::channel::<ParseCommand>();
    let (ktx, prx) = std::sync::mpsc::channel::<Parsed>();
    let blafils = statfils.to_vec();
    let _handle = std::thread::spawn(move || {
        let mut qit = line::QuickIt::new(path, vocab2, blafils);
        //vocab2.lock().unwrap();
        loop {
            if let Ok(cmd) = crx.recv() {
                match cmd {
                    ParseCommand::Parse(mut l) => {
                        while l > 0 {
                            let mut parsed: Vec<message::Overview> = vec![];
                            let mut b = 100;
                            while let Some(chunk) = qit.next() {
                                parsed.push(chunk);
                                l-=1;
                                b-=1;
                                if l == 0 || b == 0 { break; }
                            };
                            if parsed.is_empty() {
                                ktx.send(Parsed::Done).unwrap();
                                break;
                            }
                            else {
                                ktx.send(Parsed::Chunk(parsed)).unwrap();
                            }
                        }
                    },
                    ParseCommand::Stop => break,
                }
            }
        }
    });

    loop {
        if app.parsing != ParseStatus::Done {
            if let ParseStatus::Parsing(target) = app.parsing {
                if let Ok(parsed) = prx.try_recv() {
                    match parsed {
                        Parsed::Chunk(c) => {
                            if c.len() > 0 {
                                let left = target - c.len();
                                app.parsing = if left > 0 { ParseStatus::Parsing(left) } else { ParseStatus::Waiting };
                                app.items.extend(c);
                                app.selected = Some(app.items.len() - 1);
                            }
                        },
                        Parsed::Done => {
                            app.parsing = ParseStatus::Done;
                        },
                    }
                };
            }
        }

        terminal.draw(|f| ui::ui(f, &mut app, statfils, header, &vocab))?;

        if event::poll(if app.parsing != ParseStatus::Done { Duration::from_millis(10) } else { Duration::from_secs(1) })? {
            if let Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (KeyModifiers::NONE, KeyCode::Char('q')) => return Ok(()),
                    (KeyModifiers::NONE, KeyCode::Down | KeyCode::Char('j')) => app.next(&ctx, 1),
                    (KeyModifiers::NONE, KeyCode::Up | KeyCode::Char('k')) => app.previous(1),
                    (KeyModifiers::NONE, KeyCode::PageDown) => app.next(&ctx, 10),
                    (KeyModifiers::NONE, KeyCode::PageUp) => app.previous(10),
                    (KeyModifiers::NONE, KeyCode::Right | KeyCode::Char('l')) => {
                        app.focus = match app.focus {
                            Focus::MMOVERVIEW => {
                                match app.selected {
                                    Some(_) => Focus::MOVERVIEW,
                                    None => Focus::MMOVERVIEW,
                                }
                            },
                            Focus::MOVERVIEW => {
                                match app.mstate.state.selected() {
                                    Some(_) => Focus::MDETAILS,
                                    None => Focus::MOVERVIEW
                                }
                            },
                            Focus::MDETAILS => Focus::MDETAILS,
                        }
                    },
                    (KeyModifiers::NONE, KeyCode::Left | KeyCode::Char('h')) => {
                        app.focus = match app.focus {
                            Focus::MMOVERVIEW => Focus::MMOVERVIEW,
                            Focus::MOVERVIEW => Focus::MMOVERVIEW,
                            Focus::MDETAILS => Focus::MOVERVIEW,
                        }
                    },
                    (KeyModifiers::NONE, KeyCode::Char('0')) => app.target(&ctx, 0),
                    (KeyModifiers::SHIFT, KeyCode::Char('G')) => app.target(&ctx, app.items.len().saturating_sub(1)),
                    (KeyModifiers::NONE, KeyCode::Char('d')) => app.target(&ctx, usize::MAX - 1),
                    (KeyModifiers::SHIFT, KeyCode::Char('H')) => app.help = true,
                    (KeyModifiers::NONE, KeyCode::Esc) => app.help = false,
                    (_, _) => {},
                }
            }
        }
    }

    _handle.join().unwrap();
}
