pub mod render;
pub mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};
use crate::args;

pub fn run() -> Result<(), io::Error> {
    let a = args::parse_args();
    let stdout = io::stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    // set the tickrate

    loop {
        terminal.draw(|f| {
            ui::ui(f, &a);
        })?;
        // handle events, exit on q, poll for events
        let event = event::poll(Duration::from_millis(100))?;
        // check if an event was received
        if event {
            // read the event
            let e = event::read()?;
            match e {
                // exit on q
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    // restore terminal important
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
