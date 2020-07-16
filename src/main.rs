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
    #[structopt(name = "PATH(S)", required = true, min_values = 1, parse(from_os_str))]
    paths: Vec<std::path::PathBuf>,
    #[structopt(short)]
    long: bool,
    #[structopt(short, long)]
    recursive: bool
}

fn main() {
    let args = Opt::from_args();
    println!("{:#?}", args);
}
