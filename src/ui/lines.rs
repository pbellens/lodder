use crate::app;
use crate::stats;
use tui::style::Color;
use tui::style::Modifier;
use tui::symbols;
use tui::text::Span;
use tui::widgets::Axis;
use tui::widgets::Chart;
use tui::widgets::Dataset;
use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders},
    Frame,
};

pub(crate) fn draw<B>(
    f: &mut Frame<B>,
    app: &mut app::App,
    area: Rect,
    statfils: &[stats::StatsCfg],
    width: i64,
) where
    B: Backend,
{
    let middle = app.selected.map(|m| m as i64).unwrap_or(0);
    let bounds = (middle - width, middle + width);
    let lower: usize = bounds.0.try_into().unwrap_or(0);
    let backing = statfils
        .iter()
        .map(|s| {
            app
                .items
                .iter()
                .enumerate()
                .skip(bounds.0.try_into().unwrap_or(0))
                .take(TryInto::<usize>::try_into(bounds.1).unwrap() - lower)
                .map(|(i, oview)| {
                    (
                        i as f64,
                        oview
                            .stats
                            .get(s.id.as_str())
                            .or(Some(&0.0))
                            .unwrap()
                            .clone()
                    )})
                .collect::<Vec<_>>() })
        .collect::<Vec<_>>();

    let datarange = backing.iter()
        .flat_map(|ss| ss.iter())
        .fold(
            (f64::MAX, f64::MIN), 
            |acc, xy| { (acc.0.min(xy.1), acc.1.max(xy.1)) });

    let datasets = backing
        .iter()
        .zip(statfils)
        .map(|(d, s)| {
            Dataset::default()
                .name(&s.title)
                .marker(symbols::Marker::Block)
                .style(Style::default().fg(s.color))
                .data(&d) })
        .collect::<Vec<_>>();

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([bounds.0 as f64, bounds.1 as f64])
                .labels(
                    vec![
                        Span::raw(bounds.0.to_string()),
                        Span::raw(app.selected.unwrap_or(0).to_string()),
                        Span::raw(bounds.1.to_string())]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled((datarange.0.round() as i64).to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled((datarange.1.round() as i64).to_string(), Style::default().add_modifier(Modifier::BOLD)),
                ])
                .bounds([datarange.0, datarange.1]),
        );

    f.render_widget(chart, area);
}
