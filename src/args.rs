use clap::Parser;

/// Simple ascii-clock that also is able to do countdowns
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The name of the timer
    #[arg(short, long, default_value = "ASCII-CLOCK")]
    pub name: String,
    /// The time to count down from
    #[arg(short, long, default_value = "0")]
    pub time: String,
    /// The tickrate of the clock
    #[arg(long, default_value = "1000")]
    pub tickrate: usize,
}

pub fn parse_args() -> Args {
    Args::parse()
}

