#[derive(Debug)]
pub enum OutputFormat {
    JSON,
    PlainText
}
pub const RESET_TERMINAL: &str = "\x1b[0m";
pub const RED_COLOR: &str = "\x1b[31m";
pub const YELLOW_COLOR: &str = "\x1b[33m";
pub const BLUE_COLOR: &str = "\x1b[34m";
pub const BOLD: &str = "\x1b[1m";
