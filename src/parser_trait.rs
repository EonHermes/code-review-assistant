use crate::types::AnalysisResult;
use std::path::Path;

pub trait LanguageParser {
    fn parse_project(&mut self, root: &Path, result: &mut AnalysisResult) -> crate::errors::Result<()>;
}
