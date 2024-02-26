
struct Cli {
    pattern: String,
    path: String,
    options: Vec<String>,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("Pattern missin");
    let path = std::env::args().nth(2).expect("Missing path");
    println!("Pattern: {}, path: {}", pattern, path);
}
