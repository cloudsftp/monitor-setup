use std::process::Command;

use iter_tools::Itertools;

fn set_output(command: &mut Command, display: &str) {
    command.arg("--output");
    command.arg(display);
}

fn disconnect(command: &mut Command, display: &str) {
    set_output(command, display);
    command.arg("--off");
}

fn connect(command: &mut Command, display: &str) {
    set_output(command, display);
    command.arg("--auto");
}

fn connect_extra(command: &mut Command, left: &str, right: &str) {
    connect(command, left);
    command.arg("--right-of");
    command.arg(right);
}

pub fn connect_workplace(displays: Vec<String>) -> std::io::Result<()> {
    let mut command = Command::new("xrandr");
    disconnect(&mut command, &displays[0]);
    connect(&mut command, &displays[1]);

    for (right, left) in displays.iter().skip(1).tuple_windows() {
        connect_extra(&mut command, left, right);
    }

    run(&mut command)
}

pub fn connect_mobile(displays: Vec<String>) -> std::io::Result<()> {
    let mut command = Command::new("xrandr");
    connect(&mut command, &displays[0]);
    for i in 1..displays.len() {
        disconnect(&mut command, &displays[i]);
    }

    run(&mut command)
}

fn run(command: &mut Command) -> std::io::Result<()> {
    println!("command: {:?}", command);

    let output = command.output()?;

    println!("stdout:\n{}", String::from_utf8(output.stdout).unwrap());
    println!("\nstderr:\n{}", String::from_utf8(output.stderr).unwrap());

    Ok(())
}
