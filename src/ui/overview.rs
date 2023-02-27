use crate::app;
use crate::app::ParseStatus;

use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};
use log::{debug, error, info, trace, warn};

pub(crate) fn draw<B>(
    f: &mut Frame<B>, 
    app: &mut app::App, 
    header: &[String],
    area: Rect) 
where
    B: Backend,
{
    let fullc = [Constraint::Length(19),
        Constraint::Length(10),
        Constraint::Min(16),
        Constraint::Min(6),
        Constraint::Min(4),
        Constraint::Min(6),
        Constraint::Min(8),
        Constraint::Min(32)];

    let (border, highlight, constraints) = match app.focus {
        app::Focus::MMOVERVIEW => {
            app.mstate = app::GrowingListState::default();
            app.lstate = app::GrowingListState::default();
            (
                Style::default().fg(Color::LightGreen),
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
                &fullc[..],
            )
        },
        _ => (
            Style::default(),
            Style::default().add_modifier(Modifier::REVERSED),
            &fullc[0..7]),
    };

    let scope: usize = (area.height - 4).into();
    let mut skip = app.skip;
    let sel = app.selected.unwrap_or(0);
    if sel < skip || sel > skip + scope {
        skip = app.selected.map(|s| s.saturating_sub(scope)).unwrap_or(0);
    }

    app.state.select(app.selected.map(|s| s.saturating_sub(skip)));
    app.skip = skip;

    //trace!("before: scope {0}, app.skip {1}, state.sel {2}", scope, app.skip, app.state.selected().unwrap_or(0));

    let t = Table::new(
            app.items
                .iter()
                .skip(skip)
                .take(2 * scope)
                .map(|item| { Row::new(item).height(1)/*.bottom_margin(1)*/ }))
        .header(
            Row::new(header
                .iter()
                .map(|e| &e[..])
                .map(|h| Cell::from(h).style(Style::default())))
                .style(Style::default().bg(Color::Green))
                .height(1))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(border)
            .title(
                format!("{}/{} ({})", 
                    app.selected.map(|s| s + 1).unwrap_or_else(|| 0), 
                    app.items.len(),
                    match app.parsing {
                        ParseStatus::Waiting => "wait",
                        ParseStatus::Parsing(_) => "parsing",
                        ParseStatus::Done => "done",
                    })))
        .highlight_style(highlight)
        .widths(constraints);
    f.render_stateful_widget(t, area, &mut app.state);

    app.window = if skip == 0 { app.window } else { Some((skip, app.selected.unwrap()) )};
}
