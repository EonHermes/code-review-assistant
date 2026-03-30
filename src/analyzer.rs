use crate::types::AnalysisResult;
use crate::config::Config;
use crate::errors::Result;
use std::path::PathBuf;

mod parser_trait;
mod rust_parser;
mod ts_parser;
mod python_parser;
pub use parser_trait::LanguageParser;
pub use rust_parser::RustParser;
pub use ts_parser::TypeScriptParser;
pub use python_parser::PythonParser;

pub struct ProjectAnalyzer {
    root: PathBuf,
    config: Config,
    result: AnalysisResult,
}

impl ProjectAnalyzer {
    pub fn new(root: &PathBuf, config: Config) -> Result<Self> {
        Ok(Self {
            root: root.clone(),
            config,
            result: AnalysisResult {
                project: crate::types::ProjectInfo {
                    name: String::new(),
                    version: None,
                    description: None,
                    authors: Vec::new(),
                    license: None,
                    homepage: None,
                    repository: None,
                    dependencies: Vec::new(),
                    dev_dependencies: Vec::new(),
                    language: String::new(),
                    build_info: crate::types::BuildInfo {
                        tool: String::new(),
                        target: None,
                        features: Vec::new(),
                    },
                },
                modules: indexmap::IndexMap::new(),
                root_module: "root".to_string(),
                api_endpoints: Vec::new(),
                schemas: std::collections::HashMap::new(),
                cross_references: Vec::new(),
                statistics: crate::types::AnalysisStats {
                    total_files: 0,
                    total_lines: 0,
                    total_modules: 0,
                    total_items: 0,
                    total_endpoints: 0,
                    languages: std::collections::HashMap::new(),
                    doc_coverage: 0.0,
                },
            },
        })
    }

    pub fn scan(&mut self) -> Result<()> {
        println!("🔍 Scanning project structure...");

        self.detect_project()?;
        self.collect_files()?;
        self.parse_files()?;
        self.build_graph()?;
        self.compute_statistics();

        println!("✅ Analysis complete: {} modules, {} items",
                 self.result.modules.len(),
                 self.result.statistics.total_items);
        Ok(())
    }

    fn detect_project(&mut self) -> Result<()> {
        if self.root.join("Cargo.toml").exists() {
            self.parse_cargo_toml()?;
            self.result.project.language = "Rust".to_string();
        } else if self.root.join("package.json").exists() {
            self.parse_package_json()?;
            self.result.project.language = "TypeScript".to_string();
        } else if self.root.join("pyproject.toml").exists() || self.root.join("setup.py").exists() {
            self.parse_pyproject()?;
            self.result.project.language = "Python".to_string();
        }

        Ok(())
    }

    fn parse_cargo_toml(&mut self) -> Result<()> {
        let content = std::fs::read_to_string(self.root.join("Cargo.toml"))?;
        let toml: toml::Value = toml::from_str(&content)
            .map_err(|e| crate::errors::AnalysisError::Parse(format!("Cargo.toml: {}", e)))?;

        if let Some(package) = toml.get("package") {
            self.result.project.name = package.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            self.result.project.version = package.get("version")
                .and_then(|v| v.as_str())
                .map(String::from);
            self.result.project.description = package.get("description")
                .and_then(|v| v.as_str())
                .map(String::from);
            self.result.project.license = package.get("license")
                .and_then(|v| v.as_str())
                .map(String::from);
            self.result.project.homepage = package.get("homepage")
                .and_then(|v| v.as_str())
                .map(String::from);
            self.result.project.repository = package.get("repository")
                .and_then(|v| v.as_str())
                .map(String::from);
            self.result.project.authors = package.get("authors")
                .and_then(|v| v.as_array())
                .unwrap_or_default()
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }

        self.result.project.build_info.tool = "cargo".to_string();
        if let Some(features) = toml.get("features").and_then(|v| v.as_table()) {
            self.result.project.build_info.features = features.keys().map(|k| k.clone()).collect();
        }

        Ok(())
    }

    fn parse_package_json(&mut self) -> Result<()> {
        let content = std::fs::read_to_string(self.root.join("package.json"))?;
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| crate::errors::AnalysisError::Parse(format!("package.json: {}", e)))?;

        self.result.project.name = json.get("name")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_default();
        self.result.project.version = json.get("version")
            .and_then(|v| v.as_str())
            .map(String::from);
        self.result.project.description = json.get("description")
            .and_then(|v| v.as_str())
            .map(String::from);
        self.result.project.license = json.get("license")
            .and_then(|v| v.as_str())
            .map(String::from);
        self.result.project.repository = json.get("repository")
            .and_then(|v| v.get("url"))
            .and_then(|v| v.as_str())
            .map(String::from);
        self.result.project.homepage = json.get("homepage")
            .and_then(|v| v.as_str())
            .map(String::from);

        if let Some(authors) = json.get("authors").and_then(|v| v.as_array()) {
            for author in authors {
                if let Some(name) = author.get("name").and_then(|v| v.as_str()) {
                    self.result.project.authors.push(name.to_string());
                }
            }
        }

        self.result.project.build_info.tool = "npm".to_string();
        Ok(())
    }

    fn parse_pyproject(&mut self) -> Result<()> {
        if let Ok(content) = std::fs::read_to_string(self.root.join("pyproject.toml")) {
            if let Ok(toml) = toml::from_str::<toml::Value>(&content) {
                let project = toml.get("project").or_else(|| {
                    toml.get("tool").and_then(|t| t.get("poetry"))
                });

                if let Some(p) = project {
                    self.result.project.name = p.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    self.result.project.version = p.get("version")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                    self.result.project.description = p.get("description")
                        .and_then(|v| v.as_str())
                        .map(String::from);
                }
            }
        }
        self.result.project.build_info.tool = "python".to_string();
        Ok(())
    }

    fn collect_files(&mut self) -> Result<()> {
        println!("📁 Collecting source files...");
        let mut file_count = 0;
        let exts = ["rs", "toml", "ts", "tsx", "js", "jsx", "json", "py"];

        for entry in walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
        {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if exts.contains(&ext) {
                    file_count += 1;
                }
            }
        }

        self.result.statistics.total_files = file_count;
        Ok(())
    }

    fn parse_files(&mut self) -> Result<()> {
        println!("🔄 Parsing source files...");

        let mut parser: Box<dyn LanguageParser> = match self.result.project.language.as_str() {
            "Rust" => Box::new(RustParser::new()),
            "TypeScript" => Box::new(TypeScriptParser::new()),
            "Python" => Box::new(PythonParser::new()),
            _ => return Err(crate::errors::AnalysisError::UnsupportedLanguage(
                format!("No parser for language: {}", self.result.project.language)
            ).into()),
        };

        parser.parse_project(&self.root, &mut self.result)?;
        Ok(())
    }

    fn build_graph(&mut self) -> Result<()> {
        println!("📊 Building dependency graph...");
        use crate::graph::DependencyGraph;
        let graph_builder = DependencyGraph::new(&self.result);
        self.result.cross_references = graph_builder.compute_references();
        Ok(())
    }

    fn compute_statistics(&mut self) {
        let mut total_items = 0;
        let mut total_lines = 0;
        let mut items_with_docs = 0;

        for module in self.result.modules.values() {
            total_items += module.items.len();
            total_lines += module.line_count;
            for item in &module.items {
                if item.documentation.as_ref().map(|d| !d.trim().is_empty()).unwrap_or(false) {
                    items_with_docs += 1;
                }
            }
        }

        self.result.statistics.total_modules = self.result.modules.len();
        self.result.statistics.total_items = total_items;
        self.result.statistics.total_lines = total_lines;
        self.result.statistics.doc_coverage = if total_items > 0 {
            items_with_docs as f64 / total_items as f64 * 100.0
        } else {
            0.0
        };
    }
}
