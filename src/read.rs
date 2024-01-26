use std::{process::Command, str::Lines};

pub struct Displays {
    pub connected: Vec<String>,
    pub disconnected: Vec<String>,
}

pub fn get_displays() -> Displays {
    let output = Command::new("xrandr")
        .arg("-q")
        .output()
        .expect("could not execute xrandr")
        .stdout;
    let output = String::from_utf8(output).expect("could not read the output of xrandr");

    let lines = output.lines();

    let get_displays_matching = |lines: Lines, state| {
        lines
            .filter(|l| l.contains(state))
            .map(|l| {
                l.split_ascii_whitespace()
                    .next()
                    .expect("each line should have at least one word")
                    .to_string()
            })
            .collect()
    };

    let connected = get_displays_matching(lines.clone(), " connected");
    let disconnected = get_displays_matching(lines, " disconnected");

    Displays {
        connected,
        disconnected,
    }
}
