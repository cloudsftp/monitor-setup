use std::process::Command;

use iter_tools::Itertools;

use crate::read::Displays;

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

pub fn connect_external(displays: Displays) -> std::io::Result<()> {
    let mut command = Command::new("xrandr");
    disconnect(&mut command, &displays.connected[0]);
    connect(&mut command, &displays.connected[1]);

    for (right, left) in displays.connected.iter().skip(1).tuple_windows() {
        connect_extra(&mut command, left, right);
    }

    for disconnected in displays.disconnected {
        disconnect(&mut command, &disconnected)
    }

    run(&mut command)
}

pub fn connect_mobile(displays: Displays) -> std::io::Result<()> {
    let mut command = Command::new("xrandr");
    connect(&mut command, &displays.connected[0]);

    for connected in displays.connected.iter().skip(1) {
        disconnect(&mut command, &connected);
    }

    for disconnected in displays.disconnected {
        disconnect(&mut command, &disconnected)
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
