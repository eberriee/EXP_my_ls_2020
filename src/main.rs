//
// EPITECH PROJECT, 2019
// main.rs
// File description:
// 
//

use structopt::StructOpt;

// On utilise la structure existante Cli et
// On dérive la Structure en rajoutant le champ <path>
// Activer debug pour pouvoir utiliser println!("{:#?}"
#[derive(StructOpt, Debug)]
// Nom par défault dans le usage
#[structopt(name = "my_ls")]
struct Opt {
    #[structopt(default_value = ".", name = "PATH(S)",
        required = false, parse(from_os_str))]
    paths: Vec<std::path::PathBuf>,
    #[structopt(short)]
    long: bool,
    #[structopt(short, long)]
    recursive: bool
}

fn  exit_error(message: &str, code: i32) -> !
{
    println!("ERROR: {}", message);
    std::process::exit(code);
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
fn  my_ls__print_file(inode: &std::fs::DirEntry, long: bool)
    -> Result<(), Box<dyn std::error::Error>>
{
    let path = inode.file_name().into_string()
        .or_else(|e| Err(format!("Box<dyn std::error::Error> {:?}", e)))?;

    print!("{}\t", path);
    println!("");
    Ok(())
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
fn  my_ls__read_dir(path: &std::path::PathBuf, recursive: bool, long: bool)
    -> Result<(), Box<dyn std::error::Error>>
{
    for inode in std::fs::read_dir(path)? {
        let inode = inode?;
        my_ls__print_file(&inode, long);
    }
    Ok(())
}

#[allow(non_snake_case)]
fn  my_ls(opt: &Opt) -> Result<(), Box<dyn std::error::Error>>
{
    for path in &opt.paths {
        if path.is_file() == true {
            match my_ls__print_file(std::fs::read_dir(path).unwrap(), opt.long) {
                Ok(_) => continue,
                Err(_) => return Err("Oops")?
            }
        } else if path.is_dir() == true {
            return my_ls__read_dir(path, opt.recursive, opt.long);
        }
    }
    Ok(())
}

#[allow(non_snake_case)]
fn  args__check_path(opt: &Opt) -> Result<bool, Box<dyn std::error::Error>>
{
    for path in &opt.paths {
        if (path.exists()) == false {
            print!("my_ls: cannot access '{}'. ", path.display());
            Err("Oops")?;
        }
        println!("{}", path.display());
    }
    Ok(true)
}


fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = Opt::from_args();

    println!("{:#?}", args);
    match args__check_path(&args) {
        Ok(_) => { return my_ls(&args); },
        Err(_) => exit_error("No such file or directory", 84)
    }
}
