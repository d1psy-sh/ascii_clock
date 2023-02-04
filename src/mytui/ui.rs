use crate::args;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::render;

pub fn ui<B: Backend>(f: &mut Frame<B>, args: &args::Args, time: &String) {
    let size = f.size();
    let text = render::render_ascii(&String::from(time.to_owned()));
    let par = Paragraph::new(
        text.split("\n")
            .into_iter()
            .map(|x| Spans::from(x))
            .collect::<Vec<Spans>>(),
    )
    .block(
        Block::default()
            // style bold
            .title(Span::styled(
                args.name.clone(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL),
    )
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: false });
    // render the widget
    f.render_widget(par, size)
}
