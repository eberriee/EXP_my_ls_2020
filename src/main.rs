//
// EPITECH PROJECT, 2019
// main.rs
// File description:
// Main file

use structopt::StructOpt;
mod my_ls;
mod usermod;

#[derive(StructOpt, Debug)]
#[structopt(name = "my_ls")]
struct Opt {
    #[structopt(default_value = ".", name = "PATH(S)",
        required = false, parse(from_os_str))]
    paths: Vec<std::path::PathBuf>,

    /// Use a more detailled reading format
    #[structopt(short, long)]
    long: bool,

    /// Recurses through directories
    #[structopt(short, long)]
    recursive: bool
}

fn main() -> ()
{
    let args = Opt::from_args();

    for path in &args.paths {
        print!("\n");
        if let Err(e) = my_ls::my_ls__run(path, args.long, args.recursive) {
            println!("my_ls: '{}' {}", path.display(), e);
        }    
        print!("\n");
    }
}
