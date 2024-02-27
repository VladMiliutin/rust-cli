use std::{path::PathBuf, io, error::Error, fs::{ReadDir, DirEntry}}; 

struct Cli {
    pattern: String,
    path: PathBuf,
    options: Vec<String>,
}

struct File {
    path: String,
    content: String,
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
    find_matches(&args.path, &args.pattern);
}

fn find_matches(path: &PathBuf, pattern: &String) -> Result<u16, std::io::Error> {
    println!("Reading path: {}", path.to_str().unwrap());
    let mut entries = std::fs::read_dir(path)?
        .map(|res| res
             .map(|e| {
                 find_matches_in_dir_or_file(e, pattern)
             })
        )
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(1)
}

fn find_matches_in_dir_or_file(entry: DirEntry, pattern: &String) -> Vec<String> {
     println!("Path: {}", entry.path().to_str().unwrap());
     let mut results: Vec<String> = Vec::new();
     if (entry.path().is_dir()) {
         let dir_matches = std::fs::read_dir(entry.path())
                      .map(|res| res.map(|e| {
                          match e {
                              Ok(dir) => find_matches_in_dir_or_file(dir, pattern),
                              Err(e) => Vec::new(),
                          }
                      }));
        match dir_matches {
            Ok(v) => {
                let mut vec: Vec<String> = v.flat_map(|f| f.into_iter()).collect::<Vec<String>>();
                results.append(&mut vec);
            },
            Err(e) => println!("Failed list dir, {:?}", e),
        }
     } else {
         let matches = std::fs::read_to_string(entry.path())
            .map(|content| find_matches_in_file(&content, pattern));
         match(matches) {
             Ok(v) => {
                 let mut borrowed_v = v;
                 results.append(&mut borrowed_v);
             }
             Err(e) => println!("Failed to read file, {:?}", e),
         };
     }

     results
}

fn find_matches_in_file(content: &String, pattern: &String) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    for line in content.lines() {
       if (line.contains(pattern)) {
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
