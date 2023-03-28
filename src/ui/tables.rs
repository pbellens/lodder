use crate::{app, vocab};
use crate::stats;
use std::iter;
use tui::widgets::BarChart;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders},
    Frame,
};

pub(crate) fn draw<B>(
    f: &mut Frame<B>,
    app: &mut app::App,
    vocab: &vocab::Vocabulary,
    area: Rect,
    statfils: &[stats::StatsCfg],
) where
    B: Backend,
{
    let constraints = iter::repeat(100 / statfils.len() as u16)
        .map(|p| Constraint::Percentage(p))
        .take(statfils.len())
        .collect::<Vec<_>>();
    let slay = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .margin(0)
        .split(area);

    for l in iter::zip(slay, statfils) {
        let data = app
            .items
            .iter()
            .enumerate()
            .skip(app.selected.unwrap_or(0).saturating_sub(6))
            .map(|(_i, oview)| {
                (
                    oview.header.get(l.1.idx).unwrap().string.as_ref(vocab),
                    oview
                        .stats
                        .get(l.1.id.as_str())
                        .or(Some(&0.0))
                        .unwrap()
                        .round() as u64,
                )
            })
            //.skip({
            //    let idx = app.skip + app.selected.unwrap_or(0);
            //    if idx > 4 { idx - 4 } else { 0 }
            //})
            .take(32)
            .collect::<Vec<_>>();

        let barchart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(l.1.title.as_str()),
            )
            .data(&data[..])
            .bar_width(3)
            .bar_style(Style::default().fg(l.1.color))
            .value_style(Style::default().fg(l.1.color).bg(l.1.color));
        f.render_widget(barchart, l.0);
    }
}
