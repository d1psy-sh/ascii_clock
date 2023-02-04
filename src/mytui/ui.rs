use crate::args;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::render;

pub fn ui<B: Backend>(f: &mut Frame<B>, args: &args::Args) {
    let size = f.size();
    let binding = "01:02:03";
    let text = render::render_ascii(&String::from(binding));
    let par = Paragraph::new(
        text.split("\n")
            .into_iter()
            .map(|x| Spans::from(x))
            .collect::<Vec<Spans>>(),
    )
    .block(
        Block::default()
            .title(args.name.clone())
            .borders(Borders::ALL),
    )
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    // render the widget
    f.render_widget(par, size)
}
