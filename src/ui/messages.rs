use crate::app;
use crate::vocab;
use std::sync::Arc;
use std::sync::Mutex;
use tui::{
    layout::Rect,
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    text::Spans,
    Frame,
};

pub(crate) fn draw<B>(
    f: &mut Frame<B>, 
    app: &mut app::App, 
    area: Rect, 
    voc: &Arc<Mutex<vocab::Vocab>>) 
where
    B: Backend,
{
    let vocab = voc.lock().unwrap();
    let ds = match app.state.selected()
        .map(|i| &app.items[i])
        .map(|v| v.msgs.iter().map(|s| { Spans::from(vocab.map(s.stype)) }).collect::<Vec<_>>())
    {
        Some(ds) => ds,
        None => vec![],
    };

    let (border, highlight) = match app.focus {
        app::Focus::MOVERVIEW => {
            app.mstate.length = Some(ds.len());
            app.lstate = app::GrowingListState::default();
            (Style::default().fg(Color::LightGreen),
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD))
        },
        _ => (Style::default(),
            Style::default().add_modifier(Modifier::REVERSED)),
    };

    f.render_stateful_widget(
        List::new(ds.into_iter().map(|s| ListItem::new(s)).collect::<Vec<_>>())
            .block(
                Block::default()
                .borders(Borders::ALL)
                .border_style(border))
            .highlight_style(highlight),
        area,
        &mut app.mstate.state);
}

