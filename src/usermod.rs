//
// EPITECH PROJECT, 2019
// usermod.rs
// File description:
// Usermod related functions

use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

#[allow(non_snake_case)]
fn umodtruple(mode: u32, read: u32, write: u32, execute: u32) -> String {
    let readMode: String = match mode & read {
        0 => "-".to_string(),
        _ => "r".to_string()
    };
    let writeMode: String = match mode & write {
        0 => "-".to_string(),
        _ => "w".to_string()
    };
    let execMode: String = match mode & execute {
        0 => "-".to_string(),
        _ => "x".to_string()
    };
    return readMode + &writeMode + &execMode;
}

#[allow(non_snake_case)]
pub fn my_ls__get_usermod(mode: u32) -> String
{
	let user = umodtruple(mode, S_IRUSR, S_IWUSR, S_IXUSR);
	let group = umodtruple(mode, S_IRGRP, S_IWGRP, S_IXGRP);
	let other = umodtruple(mode, S_IROTH, S_IWOTH, S_IXOTH);
	[user, group, other].join("")
}
