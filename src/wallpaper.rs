use std::{io, process::Command};

const WALLPAPER_PATH: &str = "/home/fabi/Pictures";
const WALLPAPER_NAME: &str = "forest_01.jpg";

pub fn setup() -> io::Result<()> {
    let mut command = Command::new("feh");

    let wallpaper = format!("{}/{}", WALLPAPER_PATH, WALLPAPER_NAME);
    command.arg("--bg-fill").arg(wallpaper);

    let child = command.spawn();

    child.map(|_| ())
}
