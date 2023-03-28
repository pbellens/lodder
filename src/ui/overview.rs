use crate::{app::{grow, ParseStatus}, vocab};
use crate::{app, stats::HeaderElement};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub(crate) fn draw<B>(f: &mut Frame<B>, app: &mut app::App, v: &vocab::Vocabulary, header: &[HeaderElement], area: Rect)
where
    B: Backend,
{
    let (border, highlight, constraints) = match app.focus {
        app::Focus::MMOVERVIEW => {
            app.mstate = grow::GrowingListState::default();
            app.lstate = grow::GrowingListState::default();
            (
                Style::default().fg(Color::LightGreen),
                Style::default()
                    .bg(if let app::Mode::Finding = app.mode { Color::Yellow } else { Color:: LightGreen })
                    .add_modifier(Modifier::BOLD),
                header.iter().map(|e| e.constraint).collect::<Vec<_>>(),
            )
        }
        _ => (
            Style::default(),
            Style::default().add_modifier(Modifier::REVERSED),
            header
                .iter()
                .map(|e| e.constraint)
                .take(7)
                .collect::<Vec<_>>(),
        ),
    };

    let scope: usize = (area.height - 4).into();
    let mut skip = app.skip;
    let sel = app.selected.unwrap_or(0);
    if sel < skip || sel > skip + scope {
        skip = app.selected.map(|s| s.saturating_sub(scope)).unwrap_or(0);
    }

    app.state
        .select(app.selected.map(|s| s.saturating_sub(skip)));
    app.skip = skip;

    let t = Table::new(
            app.items.iter()
                .skip(skip)
                .take(2 * scope)
                .map(|item| { 
                    Row::new(item.into_iter().map(|i| Cell::from(i.string.as_ref(v)).style(i.style))).height(1)
                }))
        .header(
            Row::new(
                header
                    .iter()
                    .map(|e| e.name.as_ref())
                    .map(|h| Cell::from(h).style(Style::default())),
            )
            .style(Style::default().bg(Color::Green))
            .height(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border)
                .title(format!(
                    "{}/{} ({})",
                    app.selected.map(|s| s + 1).unwrap_or_else(|| 0),
                    app.items.len(),
                    match app.parsing {
                        ParseStatus::Waiting => "wait",
                        ParseStatus::Parsing(_) => "parsing",
                        ParseStatus::Done => "done",
                    }
                )),
        )
        .highlight_style(highlight)
        .widths(&constraints);
    f.render_stateful_widget(t, area, &mut app.state);

    app.window = if skip == 0 {
        app.window
    } else {
        Some((skip, app.selected.unwrap()))
    };
}
