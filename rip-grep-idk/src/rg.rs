pub mod rg {
    use std::{path::PathBuf, io::Error, fs};

    pub struct Match {
        line_number: u32,
        line_text: String,
    }

    pub struct FileMatches {
        path: PathBuf,
        results: Vec<Match>,
    }

    pub fn find_dir_matches(path: PathBuf, pattern: &String) -> Result<Vec<FileMatches>, Error> {
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

    pub fn find_matches(entry: PathBuf, pattern: &String) -> Result<FileMatches, Error> {
        fs::read_to_string(&entry)
            .map(|content| find_matches_in_file(&content, pattern))
            .map(|file_matches| {
                let file_match = FileMatches { path: entry, results: file_matches };
                print_file_matches(&file_match);
                file_match
            })
    }

    pub fn print_file_matches(file: &FileMatches) {
        if !file.results.is_empty() {
            println!("{}", file.path.display());
            let iter = file.results.iter();
            iter.for_each(|e| println!("{}: {}", e.line_number, e.line_text));
        }
    }

    pub fn find_matches_in_file(content: &String, pattern: &String) -> Vec<Match> {
        let mut matches: Vec<Match> = Vec::new();

        let mut lines = content.lines();
        let mut line_number: u32 = 1;

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
}
