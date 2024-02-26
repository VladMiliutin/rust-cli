use std::{path::PathBuf, error::Error, fs::ReadDir}; 

struct Cli {
    pattern: String,
    path: PathBuf,
    options: Vec<String>,
}

fn main() {
    let mut args = std::env::args();
    let pattern = args.nth(args.len() - 2).expect("Missing: `pattern`");
    let path = args.last().expect("Missing: `path`");
    let path_buff = parse_path(&path);

    let args = Cli {
        pattern: pattern,
        path: path_buff,
        options: read_options(),
    };
    
    let content: Result<String, std::io::Error> = std::fs::read_to_string(&args.path);
}

fn find_matches(path: &PathBuf, pattern: &String) -> Result<u16, std::io::Error> {
    let dir: Result<ReadDir, std::io::Error> = std::fs::read_dir(path);
    Ok(1)
}

fn find_matches_in_file(content: &String, pattern: &String) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    for line in content.lines() {
       if (line.contains(pattern)) {
           matches.push(line.to_string());
       }
    }

    return matches;
}

fn read_options() -> Vec<String> {
    let mut options = Vec::new();
    for arg in std::env::args().skip(2) {
       options.push(arg);
    }

    return options;
}

fn parse_path(path: &String) -> PathBuf {
    PathBuf::from(path)
}
