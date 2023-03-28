use crate::app;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub(crate) fn draw<B: Backend>(f: &mut Frame<B>, app: &app::App, prefix: char, area: Rect) {
    let prompt = format!("{prefix}{cmd}", cmd = app.command);
    let input = Paragraph::new(prompt.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("ex"));
    f.render_widget(input, area);
    f.set_cursor(
        area.x + app.command.chars().count() as u16 + 1 + 1,
        // Move one line down, from the border to the input line
        area.y + 1,
    );
}
