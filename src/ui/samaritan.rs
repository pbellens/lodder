use crate::app;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub(crate) fn draw<B>(f: &mut Frame<B>, _app: &mut app::App, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from(Span::styled(
            "↓ or 'j':     advance 1 item.",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "↑ or 'k':     go back 1 item.",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "→ or 'l':     go down 1 level.",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "← or 'h':     go up 1 level.",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "esc     :     go back to view mode.",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "d       :     read all messages and jump to last one",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "t       :     use charts for stat visualization",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "f       :     use lines for stat visualization",
            Style::default(),
        )),
        Spans::from(Span::styled(
            ":       :     switch to linejump mode",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "/       :     switch to search mode",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "      'n'     :     go to next match",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "      'N'     :     go to previous match",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "0       :     jump to first message.",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "shift-G :     jump to last, parsed message",
            Style::default(),
        )),
        Spans::from(Span::styled(
            "shift-H :     show this help.",
            Style::default(),
        )),
    ];

    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("help").borders(Borders::ALL))
            //.style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left),
        area,
    );
}
