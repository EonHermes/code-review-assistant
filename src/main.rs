use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ads",
    about = "Automated Document Synthesizer - Generate comprehensive documentation from code",
    long_about = "Analyzes code repositories to produce architecture diagrams, API docs, module references, and changelogs"
)]
struct Cli {
    /// Path to the project to analyze
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Output directory for generated documentation
    #[arg(short, long, default_value = "docs")]
    output: PathBuf,

    /// Configuration file path (default: docs.yaml or .docs.yaml in project root)
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Generate changelog from git history (requires git feature)
    #[arg(short, long)]
    changelog: bool,

    /// Format for main output (markdown, html, json)
    #[arg(short, long, default_value = "markdown")]
    format: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Include test files in analysis
    #[arg(short, long)]
    include_tests: bool,
}

mod analyzer;
mod output;
mod config;
mod errors;
mod graph;
mod git_analyzer;
mod language;

use analyzer::ProjectAnalyzer;
use output::Generator;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    let config = Config::load(&cli.path, cli.config.as_ref())?;
    let mut analyzer = ProjectAnalyzer::new(&cli.path, config)?;

    println!("Analyzing project at: {}", cli.path.display());
    analyzer.scan()?;

    println!("Generating documentation to: {}", cli.output.display());
    let generator = Generator::new(&cli.output, analyzer);
    generator.generate_all()?;

    if cli.changelog {
        println!("Generating changelog...");
        generator.generate_changelog()?;
    }

    println!("✅ Documentation generated successfully!");
    Ok(())
}
