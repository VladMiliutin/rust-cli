use std::{path::PathBuf, io::Error, fs};

struct Cli {
    pattern: String,
    path: PathBuf,
    options: Vec<String>,
}

struct Match {
    line_number: u32,
    line_text: String,
}

struct FileMatches {
    path: PathBuf,
    results: Vec<Match>,
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

    if find_dir_matches(args.path, &args.pattern).is_err() {
        println!("Failed to scan {:?}", &path);
    }
}

fn find_dir_matches(path: PathBuf, pattern: &String) -> Result<Vec<FileMatches>, Error> {
    let mut results: Vec<FileMatches> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();
    dirs.push(path);

    loop {
        let dir_opt = dirs.pop();

        if dir_opt == None {
            break;
        }

        fs::read_dir(dir_opt.unwrap())?
            .map(|res| {
                res.map(|e| {
                    if e.path().is_dir() {
                        dirs.push(e.path());
                    }

                    find_matches(e.path(), pattern)
                        .map(|e| {
                            results.push(e)
                        })
                })
            })
        .last();
    }


    Ok(results)
}

fn find_matches(entry: PathBuf, pattern: &String) -> Result<FileMatches, Error> {
    fs::read_to_string(&entry)
        .map(|content| find_matches_in_file(&content, pattern))
        .map(|file_matches| {
            let file_match = FileMatches { path: entry, results: file_matches };
            print_file_matches(&file_match);
            file_match
        })
}

fn print_file_matches(file: &FileMatches) {
    if !file.results.is_empty() {
        println!("{}", file.path.display());
        let iter = file.results.iter();
        iter.for_each(|e| println!("{}: {}", e.line_number, e.line_text));
    }
}

fn find_matches_in_file(content: &String, pattern: &String) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    let mut lines = content.lines();
    let mut line_number: u32 = 0;

    loop {
        let line = lines.next();

        if line == None {
            break;
        }

        let line_text = line.unwrap();

        if line_text.contains(pattern) {
            matches.push(Match { line_number: line_number, line_text: line_text.to_string() });
        }

        line_number += 1;
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
