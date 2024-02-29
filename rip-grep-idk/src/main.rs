use std::{path::PathBuf, io::Error, fs, fs::DirEntry};

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

    if find_dir_matches(&args.path, &args.pattern).is_err() {
        println!("Failed to scan {:?}", &path);
    }
}

fn find_dir_matches(path: &PathBuf, pattern: &String) -> Result<Vec<FileMatches>, Error> {
    let mut results: Vec<FileMatches> = Vec::new();
    fs::read_dir(path)?
        .map(|res| {
            res.map(|e| find_matches(e, pattern))
                .map(|e| match e {
                    Ok(v) => {
                        let mut vec = v;
                        results.append(&mut vec)
                    },
                    Err(e) => {
                        //println!("Failed to read, should be debug here: {:?}", e)
                    },
                })
        })
        .last();

    Ok(results)
}

fn find_matches(entry: DirEntry, pattern: &String) -> Result<Vec<FileMatches>, Error> {
    let path = entry.path();
    if path.is_dir() {
        find_dir_matches(&path, pattern)
    } else {
        let mut single_file_match: Vec<FileMatches> = Vec::new();
        let file_matches = fs::read_to_string(entry.path())
            .map(|content| find_matches_in_file(&content, pattern))
            .map(|file_matches| {
                let file_match = FileMatches { path: path, results: file_matches };
                print_file_matches(&file_match);
                file_match
            });

        if file_matches.is_ok() {
            single_file_match.push(file_matches.unwrap());
        }

        Ok(single_file_match)
    }
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
