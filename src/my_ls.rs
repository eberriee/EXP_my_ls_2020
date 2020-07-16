//
// EPITECH PROJECT, 2019
// my_ls.rs
// File description:
// my_ls file

use ansi_term::Colour::Blue;
use std::os::unix::fs::PermissionsExt;
use super::usermod;

#[allow(non_snake_case)]
fn  my_ls__print_default(inode: &std::fs::DirEntry, ftype: &std::fs::FileType)
{
    let name = String::from(inode.path().file_name().unwrap().to_str().unwrap());

    if ftype.is_file() {
        print!("{:10}\t\t", name);
    } else {
        print!("{:10}\t\t", Blue.bold().paint(name + "/"));
    }
}

#[allow(non_snake_case)]
fn  my_ls__print_long(inode: &std::fs::DirEntry, ftype: &std::fs::FileType)
    -> std::io::Result<()>
{
    let name = String::from(inode.path().file_name().unwrap().to_str().unwrap());
    let metadata = inode.metadata()?;
    let size = metadata.len();
    let last_modified: chrono::DateTime<chrono::Local> = chrono::DateTime::from(metadata.modified()?);
    let mode = metadata.permissions().mode();

    if ftype.is_file() {
        println!("{} {:>5} {} {}",
            usermod::my_ls__get_usermod(mode as u32),
            size,
            last_modified.format("%b %_d %H:%M").to_string(),
            name
        );
    } else {
        println!("{} {:>5} {} {}",
            usermod::my_ls__get_usermod(mode as u32),
            size,
            last_modified.format("%b %_d %H:%M").to_string(),
            Blue.bold().paint(name + "/")
        );
    }
    Ok(())
}

#[allow(non_snake_case)]
fn  my_ls__print(inode: &std::fs::DirEntry, ftype: &std::fs::FileType, long: bool)
    -> std::io::Result<()>
{
    if long == false {
        my_ls__print_default(inode, ftype);
    } else {
        my_ls__print_long(inode, ftype)?;
    }
    Ok(())
}

#[allow(non_snake_case)]
pub fn  my_ls__run(path: &std::path::PathBuf, long: bool, recursive: bool)
    -> std::io::Result<()>
{
    let full_path = std::fs::canonicalize(&path)?;
    let full_path = String::from(full_path.to_str().unwrap());
    println!("{}:", Blue.bold().paint(full_path));
    let read_dir_path = std::fs::read_dir(path).unwrap();
    for inode in read_dir_path {
        let inode = inode.unwrap();
        let ftype = inode.file_type().unwrap();
        my_ls__print(&inode, &ftype, long)?;
        if ftype.is_dir() && recursive == true {
            print!("\n");
            my_ls__run(&inode.path(), long, recursive)?;
        }
    }
    Ok(())
}
