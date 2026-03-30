use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Invalid file: {0}")]
    InvalidFile(String),

    #[error("Analysis failed: {0}")]
    Failed(String),
}

pub type Result<T> = std::result::Result<T, AnalysisError>;
