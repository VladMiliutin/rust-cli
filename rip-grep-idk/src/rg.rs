pub mod rg {
    use std::{path::PathBuf, io::Error, fs};

    use rip_grep::OutputFormat;

    pub const RESET_TERMINAL: &str = "\x1b[0m";
    pub const RED_COLOR: &str = "\x1b[31m";
    pub const BLUE_COLOR: &str = "\x1b[34m";
    pub const BOLD: &str = "\x1b[1m";

    pub struct Match {
        pub line_number: u32,
        pub line_text: String,
    }

    pub struct FileMatches {
        pub path: PathBuf,
        pub results: Vec<Match>,
    }

    pub fn find_dir_matches(path: PathBuf, pattern: &String, output_format: OutputFormat, verbose: bool) -> Result<Vec<FileMatches>, Error> {
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

                        find_matches(e.path(), pattern, verbose)
                            .map(|e| {
                                results.push(e)
                            })
                    })
                })
            .last();
        }


        Ok(results)
    }

    pub fn find_matches(entry: PathBuf, pattern: &String, verbose: bool) -> Result<FileMatches, Error> {
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
            println!("{BLUE_COLOR}{BOLD}{}{RESET_TERMINAL}", file.path.display());
            let iter = file.results.iter();
            iter.for_each(|e| println!("{RED_COLOR}{}:{RESET_TERMINAL}{}", e.line_number, e.line_text));
        }
    }

    pub fn find_matches_in_file(content: &str, pattern: &str) -> Vec<Match> {
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

#[test]
fn find_matches() {
    let content = "Hello\nMy World\n";
    let pattern = "World";
    let result = find_matches_in_file(&content, &pattern);
    assert_eq!(result.len(), 1);
    let file_match = result.first().unwrap();
    assert_eq!(file_match.line_text, "My World\n");
}
