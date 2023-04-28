use std::error::Error;

use rekordbox_fix::RekordboxPlaylist;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "input.xml";

    let playlist = RekordboxPlaylist::from_file(input_file)?;
    println!("{:?}", playlist);

    Ok(())
}
