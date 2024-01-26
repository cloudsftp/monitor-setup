use std::io;

mod read;
mod write;

fn main() -> io::Result<()> {
    let displays = read::get_displays();

    if displays.connected.len() > 1 {
        write::connect_external(displays)
    } else {
        write::connect_mobile(displays)
    }?;

    Ok(())
}
