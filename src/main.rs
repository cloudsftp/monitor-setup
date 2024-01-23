use std::io;

mod read;
mod wallpaper;
mod write;

fn main() -> io::Result<()> {
    let displays = read::get_displays();

    if displays.len() > 1 {
        write::connect_workplace(displays)
    } else {
        write::connect_mobile(displays)
    }?;

    // wallpaper::setup()?;

    Ok(())
}
