use anyhow::{Ok, Result};
use std::{fs::{copy, remove_file, File}, io::{self, Read, Write}, os::unix::fs::FileExt};

pub fn create_file(filename: &str, text: Option<&str>) -> Result<()> {
    let mut file = File::create_new(filename)?;
 
    if let Some(t) = text {
        file.write_all(t.as_bytes())?;
    } 

    Ok(())

}

pub fn copy_file(source: &str, dst: &str) -> Result<()> {
    //TODO: handle the case where src and dst is the same when we allow overwriting
    copy(source, dst)?;
    Ok(())
}

pub fn cat_files(file1: &str, file2: &str, dst: &str) -> Result<()> {
    let buf1 = std::fs::read(file1)?;
    let buf2 = std::fs::read(file2)?;
    let mut file = File::create_new(dst)?;
    file.write_all(&buf1)?;
    file.write_all_at(&buf2, (buf1.len() + 1).try_into().unwrap())?;

    Ok(())
}

pub fn delete_file(filename: &str) -> Result<()> {
    remove_file(filename)?;
    Ok(())

}