pub mod rg;
pub mod cli;
pub mod json;

use cli::cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("Cli = {:?}", args);


    if args.is_help {
        Cli::show_help();
        return;
    }

    match rg::rg::find_dir_matches(args.path, &args.pattern, args.verbose) {
        Ok(v) => {
            rg::rg::print_vec_file_matches(&v, &args.format);
        }
        Err(err) => {
            println!("Failed to scan {:?}. Error: {:?}", args.path_str, err);
        }
    };
}

