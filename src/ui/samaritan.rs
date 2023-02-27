use crate::app;
use tui::{
    layout::{Alignment, Rect},
    backend::Backend,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    text::{Span, Spans},
    Frame,
};

pub(crate) fn draw<B>(
    f: &mut Frame<B>, 
    _app: &mut app::App, 
    area: Rect) 
where
    B: Backend,
{
    let text = vec![
        Spans::from(Span::styled("↓         advance 1 item.", Style::default())),
        Spans::from(Span::styled("↑         go back 1 item.", Style::default())),
        Spans::from(Span::styled("→         go down 1 level.", Style::default())),
        Spans::from(Span::styled("→         go up 1 level.", Style::default())),
        Spans::from(Span::styled("shift-H   show this help.", Style::default())),
        Spans::from(Span::styled("esc       go back to message view.", Style::default())),
        Spans::from(Span::styled("0         jump to first message.", Style::default())),
        Spans::from(Span::styled("d         read all messages and jump to last one", Style::default())),
        Spans::from(Span::styled("shift-G   jump to last, parsed message", Style::default())),
    ];

    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("help").borders(Borders::ALL))
            //.style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left),
        area);
}

