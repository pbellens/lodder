use crate::app;
use crate::stats;
use crate::vocab;
use std::sync::Arc;
use std::sync::Mutex;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect, Direction},
    Frame,
};

mod overview;
mod messages;
mod details;
mod samaritan;

fn areas<B: Backend>(f: &mut Frame<B>, app: &mut app::App, smalls: usize) -> Vec<Rect> {
    match app.focus {
        app::Focus::MMOVERVIEW => {
            let l1 = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .margin(0)
                .split(f.size());

            let l2cons = if smalls > 0 { vec![Constraint::Percentage(40), Constraint::Percentage(60)] } else { vec![Constraint::Percentage(100)]};
            let l2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(l2cons.as_slice())
                .margin(0)
                .split(l1[1]);
            if smalls > 0 { vec![l1[0], l2[0], l2[1]] } else { vec![l1[0], l2[0]] }
        },
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
        },
    }
}

pub fn ui<B: Backend>(
    f: &mut Frame<B>, 
    app: &mut app::App, 
    statfils: &[stats::StatsCfg], 
    header: &[String],
    vocab: &Arc<Mutex<vocab::Vocab>>) 
{
    let ars = areas(f, app, statfils.len());
                    
    overview::draw(f, app, header, ars[0]); 
    if app.help {
        samaritan::draw(f, app, ars[1]);
    } else {
        messages::draw(f, app, ars[1], vocab); 
    }

    if ars.len() > 2 {
        details::draw(f, app, ars[2], statfils);
    }
}
