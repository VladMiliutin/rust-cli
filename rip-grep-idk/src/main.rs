use std::{path::PathBuf}; 

struct Cli {
    pattern: String,
    path: PathBuf,
    options: Vec<String>,
}

fn main() {
    let mut args = std::env::args();
    let pattern = args.nth(args.len() - 2).expect("Missing: `pattern`");
    let path = args.last().expect("Missing: `path`");
    let path_buff = parsePath(&path);

    let cli_opts = Cli {
        pattern: pattern,
        path: path_buff,
        options: read_options(),
    };

    println!("Pattern: {}, path: {}", cli_opts.pattern, &path);
    for cli_opt in cli_opts.options {
       print!("{} \t", cli_opt);
    }
}
fn read_options() -> Vec<String> {
    let mut options = Vec::new();
    for arg in std::env::args().skip(2) {
       options.push(arg);
    }

    return options;
}
fn parsePath(path: &String) -> PathBuf {
    PathBuf::from(path)
}
