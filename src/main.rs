use clap::Parser;
use drywall::{
    Config, OutputFormat, RunResult, format_json, format_text, parse_output_format, run,
    validate_lang,
};
use std::process;

#[derive(Parser)]
#[command(name = "drywall", about = "Detect duplicate Rust functions")]
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

    #[arg(long, help = "Force language (only 'rust' supported)")]
    lang: Option<String>,

    #[arg(long, help = "Exclude glob patterns (repeatable)")]
    exclude: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let format = match parse_output_format(&cli.format) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(2);
        }
    };

    if let Some(lang) = &cli.lang
        && let Err(e) = validate_lang(lang)
    {
        eprintln!("error: {}", e);
        process::exit(2);
    }

    let config = Config {
        threshold: cli.threshold,
        min_lines: cli.min_lines,
        min_nodes: cli.min_nodes,
        format,
        excludes: cli.exclude,
    };

    match run(&cli.paths, &config) {
        RunResult::Clean => {
            if matches!(config.format, OutputFormat::Json) {
                println!("[]");
            }
            process::exit(0);
        }
        RunResult::Duplicates(pairs) => {
            match config.format {
                OutputFormat::Text => print!("{}", format_text(&pairs)),
                OutputFormat::Json => println!("{}", format_json(&pairs)),
            }
            process::exit(1);
        }
        RunResult::Error(msg) => {
            eprintln!("error: {}", msg);
            process::exit(2);
        }
    }
}
