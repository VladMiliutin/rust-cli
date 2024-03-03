pub mod rg;
pub mod cli;

use cli::cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("Cli = {:?}", args);


    if args.is_help {
        Cli::show_help();
        return;
    }

    if rg::rg::find_dir_matches(args.path, &args.pattern, args.format, args.verbose).is_err() {
        println!("Failed to scan {:?}", args.path_str);
    }
}

