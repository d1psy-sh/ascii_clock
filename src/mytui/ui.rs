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
    // here is some shit but this is actually a clock so i dont care
    let mut text_vec: Vec<String> = text.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let horizontal = (size.width - (2 + text_vec[1].len()) as u16) as usize;
    let vertical = (size.height - (2 + text_vec.len()) as u16) as usize;
    text_vec = render::set_margin(text_vec, horizontal, vertical);
let spans = text_vec
        .into_iter()
        .map(|x| Spans::from(x))
        .collect::<Vec<Spans>>();
    let par = Paragraph::new(spans)
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
