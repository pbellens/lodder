use crate::app;
use crate::message;
use crate::stats;
use std::iter;
use log::trace;
use tui::widgets::BarChart;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    text::{Spans, Span},
    Frame,
};

pub(crate) fn draw<B>(
    f: &mut Frame<B>, 
    app: &mut app::App, 
    area: Rect, 
    statfils: &[stats::StatsCfg])
where
    B: Backend,
{
    let ga = match (app.state.selected(), app.mstate.state.selected())
    {
        (Some(s), Some(ms)) => {
            app.items[s].msgs[ms].contents
                .iter()
                .map(|l| { 
                    Spans::from(
                        Span::styled(
                            &l.string[..], 
                            match l.format {
                                message::Format::Bold => Style::default()
                                    .add_modifier(Modifier::BOLD),
                                _ => Style::default(),
                            })) 
                })
                .collect::<Vec<_>>()
        },
        _ => vec![],
    };

    let (border, highlight) = match app.focus {
        app::Focus::MDETAILS => {
            app.lstate.length = Some(ga.len());
            (Style::default().fg(Color::LightGreen),
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD))
        },
        _ => (Style::default(),
            Style::default().add_modifier(Modifier::REVERSED)),
    };
    
    if app.focus != app::Focus::MMOVERVIEW {
        f.render_stateful_widget(
            List::new(ga.into_iter().map(|s| ListItem::new(s)).collect::<Vec<_>>())
                .block(Block::default().borders(Borders::ALL).border_style(border))
                .highlight_style(highlight),
            area,
            &mut app.lstate.state);
    }
    else
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
            let data = app.items
                .iter()
                .skip(app.skip)
                .map(|oview| {
                    (
                        &oview.no[..],
                        oview.stats
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
                .take(128)
                .collect::<Vec<_>>();

            let barchart = BarChart::default()
                .block(Block::default().borders(Borders::ALL).title(l.1.title.as_str()))
                .data(&data[..])
                .bar_width(3)
                .bar_style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Yellow).bg(Color::Yellow));
            f.render_widget(barchart, l.0);
        }
    }
}

