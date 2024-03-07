pub mod rg {
    use std::{path::PathBuf, io::Error, fs};

    use rip_grep::{OutputFormat, TERMINAL_RESET, TERMINAL_RED_TEXT, TERMINAL_YELLOW_TEXT, TERMINAL_BLUE_TEXT, TERMINAL_BOLD_TEXT};

    use crate::json::json::{Serializable, Json, JsonValue, self};

    pub struct Match {
        pub line_number: u32,
        pub line_text: String,
    }

    impl Serializable for Match {
        fn serialize(&self) -> JsonValue {
            JsonValue::JsonObject(Json::default()
                 .put_str("line_number", self.line_text.to_string())
                 .put_str("line_text", self.line_text.clone())
            )
        }

    }

    pub struct FileMatch {
        pub path: PathBuf,
        pub results: Vec<Match>,
    }

    impl Serializable for FileMatch {
        fn serialize(&self) -> JsonValue {
            JsonValue::JsonObject(Json::default()
                .put_str("path", self.path.to_str().unwrap().to_string())
                .put_vec("results", &self.results)
            )
        }
    }

    pub fn find_dir_matches(path: PathBuf, pattern: &String, verbose: bool) -> Result<Vec<FileMatch>, Error> {
        let mut results: Vec<FileMatch> = Vec::new();
        let mut dirs: Vec<PathBuf> = Vec::new();
        dirs.push(path);

        loop {
            let dir_opt = dirs.pop();

            if dir_opt == None {
                break;
            }

            let dir = dir_opt.unwrap();
            if verbose {
                println!("{TERMINAL_YELLOW_TEXT}Reading folder: {:?}{TERMINAL_RESET}", dir);
            }

            fs::read_dir(dir)?
                .map(|res| {
                    res.map(|e| {
                        if e.path().is_dir() {
                            dirs.push(e.path());
                        }

                        match find_matches(e.path(), pattern, verbose) {
                            Ok(v) => {
                                if !v.results.is_empty() {
                                    results.push(v);
                                }
                            },
                            Err(e) => {
                                if verbose {
                                    println!("{TERMINAL_RED_TEXT}Failed to read file. Error = {:?}{TERMINAL_RESET}", e);
                                }
                            },
                        }
                    })
                })
            .last();
        }


        Ok(results)
    }

    pub fn find_matches(entry: PathBuf, pattern: &String, verbose: bool) -> Result<FileMatch, Error> {
        if verbose {
            println!("{TERMINAL_YELLOW_TEXT}Reading file: {:?}{TERMINAL_RESET}", entry);
        }
        fs::read_to_string(&entry)
            .map(|content| find_matches_in_file(&content, pattern))
            .map(|file_matches| {
                let file_match = FileMatch { path: entry, results: file_matches };
                file_match
            })
    }

    pub fn print_vec_file_matches(file_match_vec: &Vec<FileMatch>, output_format: &OutputFormat) {
        match output_format {
           OutputFormat::JSON => {
               println!("{TERMINAL_BLUE_TEXT}{}{TERMINAL_RESET}", json::serialize_vec(file_match_vec).to_string());

           },
           OutputFormat::PlainText => {
               file_match_vec.into_iter().for_each(|m| print_plain_file_match(m));
           },
        }
    }

    fn print_plain_file_match(file: &FileMatch) {
        println!("{TERMINAL_BLUE_TEXT}{TERMINAL_BOLD_TEXT}{}{TERMINAL_RESET}", file.path.display());
        let iter = file.results.iter();
        iter.for_each(|e| println!("{TERMINAL_RED_TEXT}{}:{TERMINAL_RESET}{}", e.line_number, e.line_text));
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
    let content = "Hello\nMy World\t";
    let pattern = "World";
    let result = rg::find_matches_in_file(&content, &pattern);
    assert_eq!(result.len(), 1);
    let file_match = result.first().unwrap();
    assert_eq!(file_match.line_text, "My World\t");
}
