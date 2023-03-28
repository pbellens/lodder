use crate::{app::{App, ExCommand, Focus, Mode, ParseCommand}, vocab};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::{io, sync::mpsc::Sender, time::Duration};

use super::StatStyle;

pub(crate) fn handle(app: &mut App, v: &vocab::Vocabulary, d: Duration, ctx: &Sender<ParseCommand>) -> io::Result<bool> {
    if event::poll(d)? {
        if let Event::Key(key) = event::read()? {
            match &app.mode {
                Mode::Ex(cmd) => {
                    match (key.modifiers, key.code) {
                        (KeyModifiers::NONE, KeyCode::Esc) => app.mode = Mode::View,
                        (KeyModifiers::CONTROL, KeyCode::Char('u')) => app.command.clear(),
                        (KeyModifiers::NONE, KeyCode::Backspace) => {
                            app.command.pop();
                        }
                        (KeyModifiers::NONE, KeyCode::Enter) => {
                            match cmd {
                                ExCommand::Jump => match app.command.parse::<usize>() {
                                    Ok(req) => {
                                        app.mode = Mode::View;
                                        app.target(&ctx, req);
                                        app.command.clear();
                                    }
                                    Err(_) => {}
                                },
                                ExCommand::Search => {
                                    let idx = app.selected.unwrap_or(0);
                                    app.mode = Mode::Finding;
                                    if let Some(found) = app.find(v, app.items.iter().enumerate(), &app.command, idx) {
                                        app.mode = Mode::Finding;
                                        app.target(ctx, found);
                                    }
                                }
                            }
                        }
                        (_m, KeyCode::Char(c)) => app.command.extend(Some(c)),
                        _ => {}
                    }
                }
                Mode::Finding => match (key.modifiers, key.code) {
                    (KeyModifiers::NONE, KeyCode::Char('n')) => {
                        let idx = app.selected.unwrap_or(0);
                        if let Some(found) = app.find(
                            v,
                            app.items.iter().enumerate(), 
                            &app.command, 
                            idx + 1) 
                        {
                            app.target(ctx, found);
                        }
                    }
                    (KeyModifiers::SHIFT, KeyCode::Char('N')) => {
                        let idx = app.selected.unwrap_or(0);
                        if let Some(found) = app.find(
                            v,
                            app.items.iter().enumerate().rev(),
                            &app.command,
                            app.items.len().saturating_sub(idx),
                        ) {
                            app.target(ctx, found);
                        }
                    }
                    (KeyModifiers::NONE, KeyCode::Esc | KeyCode::Char('q')) => {
                        app.mode = Mode::View;
                    }
                    _ => {}
                },
                _ => match (key.modifiers, key.code) {
                    (KeyModifiers::NONE, KeyCode::Char('q')) => return Ok(false),
                    (KeyModifiers::NONE, KeyCode::Down | KeyCode::Char('j')) => app.next(&ctx, 1),
                    (KeyModifiers::NONE, KeyCode::Up | KeyCode::Char('k')) => app.previous(1),
                    (KeyModifiers::NONE, KeyCode::PageDown) => app.next(&ctx, 10),
                    (KeyModifiers::NONE, KeyCode::PageUp) => app.previous(10),
                    (KeyModifiers::NONE, KeyCode::Right | KeyCode::Char('l')) => {
                        app.focus = match app.focus {
                            Focus::MMOVERVIEW => match app.selected {
                                Some(_) => Focus::MOVERVIEW,
                                None => Focus::MMOVERVIEW,
                            },
                            Focus::MOVERVIEW => match app.mstate.state.selected() {
                                Some(_) => Focus::MDETAILS,
                                None => Focus::MOVERVIEW,
                            },
                            Focus::MDETAILS => Focus::MDETAILS,
                        }
                    }
                    (KeyModifiers::NONE, KeyCode::Left | KeyCode::Char('h')) => {
                        app.focus = match app.focus {
                            Focus::MMOVERVIEW => Focus::MMOVERVIEW,
                            Focus::MOVERVIEW => Focus::MMOVERVIEW,
                            Focus::MDETAILS => Focus::MOVERVIEW,
                        }
                    }
                    (KeyModifiers::NONE, KeyCode::Char('0')) => app.target(&ctx, 0),
                    (KeyModifiers::SHIFT, KeyCode::Char('G')) => {
                        app.target(&ctx, app.items.len().saturating_sub(1))
                    }
                    (KeyModifiers::NONE, KeyCode::Char('d')) => app.target(&ctx, usize::MAX - 1),
                    (KeyModifiers::SHIFT, KeyCode::Char('H')) => app.mode = Mode::Help,
                    (KeyModifiers::NONE, KeyCode::Char('t')) => app.statstyle = StatStyle::TABLES,
                    (KeyModifiers::NONE, KeyCode::Char('f')) => app.statstyle = StatStyle::LINES,
                    (KeyModifiers::NONE, KeyCode::Esc) => {
                        if let Mode::Ex(_) = app.mode {
                            app.command.clear();
                        }
                        app.mode = Mode::View;
                    }
                    (KeyModifiers::NONE, KeyCode::Char(':')) => {
                        app.mode = Mode::Ex(ExCommand::Jump)
                    }
                    (KeyModifiers::NONE, KeyCode::Char('/')) => {
                        app.mode = Mode::Ex(ExCommand::Search)
                    }
                    (_m, _k) => {
                        //trace!("got {m:?} + {k:?}");
                    }
                },
            }
        }
    };
    Ok(true)
}
