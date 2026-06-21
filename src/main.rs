use clap::Parser;
use drywall::{Config, execute_cli, run};
use std::process;

#[derive(Parser)]
#[command(
    name = "drywall",
    about = "Detect duplicate functions (Rust, JavaScript, TypeScript)"
)]
struct Cli {
    #[arg(help = "Paths to scan (files or directories)")]
    paths: Vec<String>,

    #[arg(long, default_value = "0.82", help = "Jaccard similarity threshold")]
    threshold: f64,

    #[arg(long, default_value = "4", help = "Minimum source lines")]
    min_lines: usize,

    #[arg(long, default_value = "20", help = "Minimum normalized AST nodes")]
    min_nodes: usize,

    #[arg(long, default_value = "text", help = "Output format: text or json")]
    format: String,

    #[arg(
        long,
        help = "Force language (rust, js, or ts); default: auto-detect by extension"
    )]
    lang: Option<String>,

    #[arg(long, help = "Exclude glob patterns (repeatable)")]
    exclude: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let config = Config {
        threshold: cli.threshold,
        min_lines: cli.min_lines,
        min_nodes: cli.min_nodes,
        excludes: cli.exclude,
        ..Config::default()
    };
    let result = execute_cli(&cli.paths, &cli.format, cli.lang.as_deref(), config, run);
    print!("{}", result.stdout);
    eprint!("{}", result.stderr);
    process::exit(result.exit_code);
}
