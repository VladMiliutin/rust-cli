use std::{path::PathBuf, io::Error, fs, fs::DirEntry};

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

    if find_matches(&args.path, &args.pattern).is_err() {
        println!("Failed to scan {:?}", &path);
    }
}

fn find_matches(path: &PathBuf, pattern: &String) -> Result<Vec<String>, Error> {
    println!("Reading path: {}", path.to_str().unwrap());
    let mut results: Vec<String> = Vec::new();
    fs::read_dir(path)?
        .map(|res| {
            res.map(|e| find_matches_in_dir_or_file(e, pattern))
                .map(|e| match e {
                    Ok(v) => {
                        let mut vec = v;
                        results.append(&mut vec)
                    },
                    Err(e) => println!("Failed to list folder: {:?}", e),
                })
        })
        .last();

    Ok(results)
}

fn find_matches_in_dir_or_file(entry: DirEntry, pattern: &String) -> Result<Vec<String>, Error> {
     if entry.path().is_dir() {
         find_matches(&entry.path(), pattern)
     } else {
        fs::read_to_string(entry.path())
            .map(|content| find_matches_in_file(&content, pattern))
     }
}

fn find_matches_in_file(content: &String, pattern: &String) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    for line in content.lines() {
       if line.contains(pattern) {
           matches.push(line.to_string());
           println!("Line: {}", line);
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
