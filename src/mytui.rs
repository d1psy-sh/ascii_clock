pub mod render;
pub mod ui;

use crate::{app, args};
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

pub fn run() -> Result<(), io::Error> {
    let a = args::parse_args();
    let stdout = io::stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    // set the tickrate
    if a.time == "0" {
        clock(&mut terminal, &a)?;
    } else {
        // start timer with a.time
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let d = app::parse_time(&a.time);
        let duration = match d {
            Ok(duration) => duration,
            Err(_) => {
                println!("error parsing time");
                std::process::exit(1);
            }
        };
        thread::spawn(move ||
            // count down here and send time left to channel
            for i in 0..duration {
                let time = duration - i;
                let time = app::countdown(time);
                tx.send(time).unwrap();
                thread::sleep(Duration::from_millis(a.tickrate as u64));
            }
        );
        countdown(&mut terminal, &a, rx)?;
    }
    // restore terminal important
    terminal.clear()?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn clock(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    a: &args::Args,
) -> Result<(), io::Error> {
    loop {
        let time = app::clock();
        terminal.draw(|f| {
            ui::ui(f, &a, &time);
        })?;
        // handle events, exit on q, poll for events
        let event = event::poll(Duration::from_millis(10))?;
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
    Ok(())
}

fn countdown(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    a: &args::Args,
    rx: Receiver<String>,
) -> Result<(), io::Error> {
    loop {
        // time from timer now
        let received_time = rx.recv();
        let time = match received_time {
            Ok(time) => time,
            Err(_) => "00:00:00".to_string(),
        };

        terminal.draw(|f| {
            ui::ui(f, &a, &time);
        })?;
        if time == "00:00:00" {
            break;
        }
        // handle events, exit on q, poll for events
        let event = event::poll(Duration::from_millis(10))?;
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
    Ok(())
}
