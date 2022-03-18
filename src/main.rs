use std::{
    error::Error,
    fs::{read, read_dir, write},
    path::{Path, PathBuf},
};

use time::OffsetDateTime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(std::env::var("IPSW_DIR").unwrap_or_else(|e| {
        eprintln!("{}", e);
        "./".to_string()
    }));

    let latest = latest(&path);
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

    path.push(format!("ipsw-{}.xml", OffsetDateTime::now_utc().date()));

    write(path, new)?;

    Ok(())
}

fn latest(path: &Path) -> Option<PathBuf> {
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
