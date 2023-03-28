use crate::app::{self, grow};
use crate::vocab;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub(crate) fn draw<B>(
    f: &mut Frame<B>,
    app: &mut app::App,
    area: Rect,
    vocabulary: &vocab::Vocabulary,
) where
    B: Backend,
{
    let ds = match app
        .state
        .selected()
        .map(|i| &app.items[app.skip + i])
        .map(|v| {
            v.msgs
                .iter()
                .map(|s| Spans::from(vocabulary.map(s.stype)))
                .collect::<Vec<_>>()
        }) {
        Some(ds) => ds,
        None => vec![],
    };

    let (border, highlight) = match app.focus {
        app::Focus::MOVERVIEW => {
            app.mstate.length = Some(ds.len());
            app.lstate = grow::GrowingListState::default();
            (
                Style::default().fg(Color::LightGreen),
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
        }
        _ => (
            Style::default(),
            Style::default().add_modifier(Modifier::REVERSED),
        ),
    };

    f.render_stateful_widget(
        List::new(ds.into_iter().map(|s| ListItem::new(s)).collect::<Vec<_>>())
            .block(Block::default().borders(Borders::ALL).border_style(border))
            .highlight_style(highlight),
        area,
        &mut app.mstate.state,
    );
}
