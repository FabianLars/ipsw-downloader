use std::{
    error::Error,
    fs::{read, read_dir, write},
    path::PathBuf,
};

use time::OffsetDateTime;

fn main() -> Result<(), Box<dyn Error>> {
    let latest = latest();
    let new = reqwest::blocking::get(
        "https://mesu.apple.com/assets/macos/com_apple_macOSIPSW/com_apple_macOSIPSW.xml",
    )?
    .bytes()?;

    if let Some(latest) = latest {
        let latest = read(latest)?;
        if latest.len() == new.len() && latest == new {
            return Ok(());
        }
    }

    write(
        format!("ipsw-{}.xml", OffsetDateTime::now_utc().date()),
        new,
    )?;

    Ok(())
}

fn latest() -> Option<PathBuf> {
    let path = std::env::var("IPSW_DIR").unwrap_or_else(|_| "./".to_string());

    let mut paths = read_dir(path)
        .unwrap()
        .filter_map(|v| v.ok())
        .map(|v| v.path())
        .filter(|v| v.extension().unwrap_or_default() == "xml")
        .filter(|v| {
            v.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .starts_with("ipsw-")
        })
        .collect::<Vec<_>>();

    paths.sort();

    paths.pop()
}
