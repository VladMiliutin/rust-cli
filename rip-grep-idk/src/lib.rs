#[derive(Debug)]
pub enum OutputFormat {
    JSON,
    PlainText
}
pub const TERMINAL_RESET: &str = "\x1b[0m";
pub const TERMINAL_RED_TEXT: &str = "\x1b[31m";
pub const TERMINAL_YELLOW_TEXT: &str = "\x1b[33m";
pub const TERMINAL_BLUE_TEXT: &str = "\x1b[34m";
pub const TERMINAL_BOLD_TEXT: &str = "\x1b[1m";
