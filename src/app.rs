use std::{fmt::Error, time};

fn clock() -> String {
    let now = time::SystemTime::now();
    let since_the_epoch = now
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards");
    let in_seconds = since_the_epoch.as_secs();
    let in_hours = in_seconds / 3600;
    let in_minutes = (in_seconds % 3600) / 60;
    let in_seconds = in_seconds % 60;
    format!("{:02}:{:02}:{:02}", in_hours, in_minutes, in_seconds)
}

/// give the time left as a formated string
fn countdown(time: usize) -> String {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn parse_time(time: String) -> Result<usize, Error> {
    // parse the time allow two formats one format is "1d 2h 4m 10s" and one format is "1000"
    // which is interpreted as seconds
    match time.parse::<usize>() {
        Ok(time) => Ok(time),
        Err(_) => {
            let mut time = time;
            let mut seconds = 0;
            let mut minutes = 0;
            let mut hours = 0;
            let mut days = 0;
            // check if there are seconds
            if time.contains("s") {
                let split = time.split("s").collect::<Vec<&str>>();
                match split[0].parse::<usize>() {
                    Ok(time) => seconds = time,
                    Err(_) => return Err(Error),
                }
                time = split[1].to_string();
            }
            // check if there are minutes
            if time.contains("m") {
                let split = time.split("m").collect::<Vec<&str>>();
                match split[0].parse::<usize>() {
                    Ok(time) => minutes = time,
                    Err(_) => return Err(Error),
                }
                time = split[1].to_string();
            }
            // check if there are hours
            if time.contains("h") {
                let split = time.split("h").collect::<Vec<&str>>();
                match split[0].parse::<usize>() {
                    Ok(time) => hours = time,
                    Err(_) => return Err(Error),
                }
                time = split[1].to_string();
            }
            // check if there are days
            if time.contains("d") {
                let split = time.split("d").collect::<Vec<&str>>();
                match split[0].parse::<usize>() {
                    Ok(time) => days = time,
                    Err(_) => return Err(Error),
                }
            }
            // convert all the time to seconds
            seconds += minutes * 60;
            seconds += hours * 3600;
            seconds += days * 86400;
            Ok(seconds)
        }
    }
}
