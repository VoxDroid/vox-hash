use crate::errors::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn read_lines<P>(path: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let path_ref = path.as_ref();
    let file = File::open(path_ref).map_err(|e| {
        crate::errors::AppError::IoContext(format!(
            "Failed to open file '{}': {}",
            path_ref.display(),
            e
        ))
    })?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter_map(|line| line.ok().map(|l| l.trim().to_string()))
        .filter(|l| !l.is_empty())
        .collect();
    Ok(lines)
}

pub fn write_to_file<P>(path: P, content: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path_ref = path.as_ref();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path_ref)
        .map_err(|e| {
            crate::errors::AppError::IoContext(format!(
                "Failed to write to file '{}': {}",
                path_ref.display(),
                e
            ))
        })?;
    writeln!(file, "{}", content)?;
    Ok(())
}
