use std::{env::temp_dir, fs::File, io::Write};

use anyhow::Result;

pub fn run_webview() -> Result<()> {
    let temp_dir = temp_dir();
    let file_path = temp_dir.join("temp_index.html");

    let content = include_str!("../index.html");

    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    let url = format!("file:///{}", file_path.to_str().unwrap().replace('\\', "/"));
    webbrowser::open(&url)?;

    Ok(())
}
