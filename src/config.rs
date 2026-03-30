use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub exclude: HashSet<String>,
    pub include: HashSet<String>,
    pub language: LanguageConfig,
    pub api_detection: ApiDetectionConfig,
    pub graph: GraphConfig,
    pub changelog: ChangelogConfig,
}

impl Config {
    pub fn load(project_root: &PathBuf, custom_path: Option<&PathBuf>) -> anyhow::Result<Self> {
        let default_paths = [
            project_root.join("docs.yaml"),
            project_root.join(".docs.yaml"),
        ];

        let config_path = if let Some(custom) = custom_path {
            custom.clone()
        } else {
            default_paths
                .iter()
                .find(|p| p.exists())
                .cloned()
                .unwrap_or_else(|| project_root.join("docs.yaml"))
        };

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let mut config: Config = serde_yaml::from_str(&content)?;
            // Ensure we have defaults
            config.set_defaults();
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    fn set_defaults(&mut self) {
        if self.exclude.is_empty() {
            self.exclude.insert("target".to_string());
            self.exclude.insert("node_modules".to_string());
            self.exclude.insert(".git".to_string());
            self.exclude.insert("dist".to_string());
            self.exclude.insert("build".to_string());
        }
        if self.include.is_empty() {
            self.include.insert("src".to_string());
            self.include.insert("lib".to_string());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub rust: RustConfig,
    pub typescript: TypeScriptConfig,
    pub python: PythonConfig,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            rust: RustConfig::default(),
            typescript: TypeScriptConfig::default(),
            python: PythonConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustConfig {
    pub analyze_macros: bool,
    pub include_docs_comments: bool,
    pub extract_tests: bool,
}

impl Default for RustConfig {
    fn default() -> Self {
        Self {
            analyze_macros: true,
            include_docs_comments: true,
            extract_tests: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptConfig {
    pub extract_jSDoc: bool,
    pub analyze_frameworks: bool,
    pub include_declarations: bool,
}

impl Default for TypeScriptConfig {
    fn default() -> Self {
        Self {
            extract_jSDoc: true,
            analyze_frameworks: true,
            include_declarations: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    pub analyze_type_hints: bool,
    pub extract_docstrings: bool,
    pub detect_frameworks: bool,
}

impl Default for PythonConfig {
    fn default() -> Self {
        Self {
            analyze_type_hints: true,
            extract_docstrings: true,
            detect_frameworks: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDetectionConfig {
    pub auto: bool,
    pub frameworks: Vec<String>,
    pub extract_schemas: bool,
}

impl Default for ApiDetectionConfig {
    fn default() -> Self {
        Self {
            auto: true,
            frameworks: vec![
                "actix-web".to_string(),
                "rocket".to_string(),
                "axum".to_string(),
                "express".to_string(),
                "fastapi".to_string(),
                "flask".to_string(),
            ],
            extract_schemas: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    pub format: String,
    pub include_external: bool,
    pub max_depth: Option<usize>,
    pub cluster_modules: bool,
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            format: "svg".to_string(),
            include_external: false,
            max_depth: None,
            cluster_modules: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogConfig {
    pub ConventionalCommits: bool,
    pub max_entries: Option<usize>,
    pub include_links: bool,
    pub since_tag: Option<String>,
}

impl Default for ChangelogConfig {
    fn default() -> Self {
        Self {
            ConventionalCommits: true,
            max_entries: None,
            include_links: true,
            since_tag: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exclude: HashSet::new(),
            include: HashSet::new(),
            language: LanguageConfig::default(),
            api_detection: ApiDetectionConfig::default(),
            graph: GraphConfig::default(),
            changelog: ChangelogConfig::default(),
        }
    }
}
