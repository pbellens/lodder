use crate::{app, vocab};
use tui::layout::Constraint;
use tui::widgets::Cell;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

pub(crate) fn draw<B>(f: &mut Frame<B>, app: &mut app::App, vocab: &vocab::Vocabulary, area: Rect)
where
    B: Backend,
{
    let ga = match (app.state.selected(), app.mstate.state.selected()) {
        (Some(s), Some(ms)) => app.items[app.skip + s].msgs[ms]
            .contents
            .iter()
            .map(|ml| {
                Row::new([
                    Cell::from(ml.key.string.as_ref(vocab))
                        .style(Style::default().add_modifier(Modifier::BOLD)),
                    Cell::from(ml.value.string.as_ref(vocab)).style(Style::default()),
                ])
                .height(1)
            })
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    let (border, highlight) = match app.focus {
        app::Focus::MDETAILS => {
            app.lstate.length = Some(ga.len());
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

    //f.render_stateful_widget(
    //    List::new(ga.into_iter().map(|s| ListItem::new(s)).collect::<Vec<_>>())
    //        .block(Block::default().borders(Borders::ALL).border_style(border))
    //        .highlight_style(highlight),
    //    area,
    //    &mut app.lstate.state);

    let t = Table::new(ga)
        .header(
            Row::new(
                ["key", "value"]
                    .iter()
                    .map(|e| &e[..])
                    .map(|h| Cell::from(h).style(Style::default())),
            )
            .style(Style::default().bg(Color::Green))
            .height(1),
        )
        .block(Block::default().borders(Borders::ALL).border_style(border))
        .highlight_style(highlight)
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)]);
    f.render_stateful_widget(t, area, &mut app.lstate.state);
}
