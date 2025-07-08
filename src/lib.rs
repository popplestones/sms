pub mod providers;

use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    process::Command,
};

use tempfile::NamedTempFile;

pub fn read_phone_numbers<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().map_while(Result::ok).collect()
}

pub fn open_editor_with_template(template: &str) -> io::Result<String> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(
        temp_file.as_file_mut(),
        "# Enter your message below. Lines starting with '#' will be ignored."
    )?;
    writeln!(temp_file.as_file_mut(), "{template}")?;

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".into());
    Command::new(editor).arg(temp_file.path()).status()?;

    let mut content = String::new();
    File::open(temp_file.path())?.read_to_string(&mut content)?;
    let message = content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    Ok(message)
}
