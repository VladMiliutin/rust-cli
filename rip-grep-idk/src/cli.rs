pub mod cli {
    use std::{path::PathBuf};

    pub struct Cli {
        pub pattern: String,
        pub path: PathBuf,
        pub path_str: String,
        pub options: Vec<String>,
    }

    impl Cli {
        pub fn parse() -> Cli {
            let mut args = std::env::args();
            let pattern = args.nth(args.len() - 2).expect("Missing: `pattern`");
            let path = args.last().expect("Missing: `path`");
            let path_buff = parse_path(&path);

            Cli {
                path_str: path,
                pattern: pattern,
                path: path_buff,
                options: read_options(),
            }
        }
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
}
