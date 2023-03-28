use crate::{app, vocab};
use crate::stats;
use crate::stats::HeaderElement;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

mod command;
mod details;
mod messages;
mod overview;
mod samaritan;
mod tables;
mod lines;

fn areas<B: Backend>(f: &mut Frame<B>, app: &mut app::App, smalls: usize) -> Vec<Rect> {
    match app.focus {
        app::Focus::MMOVERVIEW => {
            let l1 = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .margin(0)
                .split(f.size());

            let l2cons = if smalls > 0 {
                vec![Constraint::Percentage(40), Constraint::Percentage(60)]
            } else {
                vec![Constraint::Percentage(100)]
            };
            let l2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(l2cons.as_slice())
                .margin(0)
                .split(l1[1]);
            if smalls > 0 {
                vec![l1[0], l2[0], l2[1]]
            } else {
                vec![l1[0], l2[0]]
            }
        }
        _ => {
            let l1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .margin(0)
                .split(f.size());

            let l2 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .margin(0)
                .split(l1[0]);
            vec![l2[0], l2[1], l1[1]]
        }
    }
}

pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    app: &mut app::App,
    vocab: &vocab::Vocabulary,
    statfils: &[stats::StatsCfg],
    header: &[HeaderElement],
) {
    let ars = areas(f, app, statfils.len());

    overview::draw(f, app, vocab, header, ars[0]);
    match app.mode {
        app::Mode::Help => samaritan::draw(f, app, ars[1]),
        app::Mode::Ex(cmd) => command::draw(
            f,
            app,
            if cmd == app::ExCommand::Jump {
                ':'
            } else {
                '/'
            },
            ars[1],
        ),
        app::Mode::Finding => command::draw(f, app, '/', ars[1]),
        _ => messages::draw(f, app, ars[1], vocab),
    };

    if ars.len() > 2 {
        if app.focus != app::Focus::MMOVERVIEW {
            details::draw(f, app, vocab, ars[2]);
        } else {
            match app.statstyle {
                app::StatStyle::NOTHING => {}
                app::StatStyle::LINES => lines::draw(f, app, ars[2], statfils, 10),
                app::StatStyle::TABLES => tables::draw(f, app, vocab, ars[2], statfils),
            }
        }
    }
}
