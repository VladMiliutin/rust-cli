pub mod rg;
pub mod cli;

use cli::cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("Cli = {:?}", args);
    if rg::rg::find_dir_matches(args.path, &args.pattern).is_err() {
        println!("Failed to scan {:?}", args.path_str);
    }
}

