use bytes::Bytes;
use flate2::read::GzDecoder;
use std::error::Error;
use std::io::Cursor;
use tar::Archive;

pub fn decompress_tar(data: Bytes) -> Result<(), Box<dyn Error>> {
    // Get the current directory of the application
    let mut dir = std::env::current_dir()?;
    dir.push("temp");

    println!("Temp directory: {:?}", dir);
    println!("Extracting tar file...");

    let tar = GzDecoder::new(Cursor::new(data));
    let mut archive = Archive::new(tar);
    archive.unpack(dir)?;

    println!("Tar file decompressed");

    Ok(())
}
