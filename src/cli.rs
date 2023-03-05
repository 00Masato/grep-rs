use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub search_word: Option<String>,
    pub search_target: Option<String>,

    /// Print num lines of trailing context after each match.
    #[arg(short = 'A', long, default_value_t = 0)]
    pub after_context: u8,

    /// Print num lines of leading context before each match.
    #[arg(short = 'B', long, default_value_t = 0)]
    pub before_context: u8,

    /// Print num lines of leading and trailing context surrounding each match.
    #[arg(short = 'C', long, default_value_t = 0)]
    pub context: u8,
}
