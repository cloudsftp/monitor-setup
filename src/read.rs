use std::process::Command;

pub fn get_displays() -> Vec<String> {
    let output = Command::new("xrandr")
        .arg("-q")
        .output()
        .expect("could not execute xrandr")
        .stdout;
    let output = String::from_utf8(output).expect("could not read the output of xrandr");

    let displays = output
        .split("\n")
        .filter(|l| l.contains(" connected"))
        .map(|l| {
            l.split_ascii_whitespace()
                .next()
                .expect("each line should have at least one word")
                .to_string()
        })
        .collect();

    displays
}
