use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    println!("pattern: {:?}, path: {:?}", args.pattern, &args.path);

    grrs::find_matches(&content, &args.pattern, std::io::stdout());
    Ok(())
}
