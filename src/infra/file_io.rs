use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use crate::errors::Result;

pub fn read_lines<P>(path: P) -> Result<Vec<String>>
where P: AsRef<Path> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .filter_map(|line| line.ok().map(|l| l.trim().to_string()))
        .filter(|l| !l.is_empty())
        .collect();
    Ok(lines)
}

pub fn write_to_file<P>(path: P, content: &str) -> Result<()>
where P: AsRef<Path> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}
