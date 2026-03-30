use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use indexmap::IndexMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Vec<Dependency>,
    pub language: String,
    pub build_info: BuildInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub source: Option<String>,
    pub is_dev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInfo {
    pub tool: String, // "cargo", "npm", "poetry", etc.
    pub target: Option<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub path: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub items: Vec<ModuleItem>,
    pub documentation: Option<String>,
    pub line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: String,
    pub name: String,
    pub kind: ItemKind,
    pub visibility: Visibility,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub attributes: Vec<String>,
    pub line: usize,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Struct,
    Enum,
    Trait,
    Function,
    Constant,
    TypeAlias,
    Module,
    Class,
    Method,
    Interface,
    Variable,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Visibility {
    Public,
    Private,
    Restricted(String), // e.g., crate, self, super
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
    pub responses: Vec<Response>,
    pub tags: Vec<String>,
    pub source_file: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub location: String, // query, path, header, body
    pub required: bool,
    pub schema: Schema,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub required: bool,
    pub schema: Schema,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status_code: String,
    pub description: String,
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub r#type: String,
    pub format: Option<String>,
    pub items: Option<Box<Schema>>,
    pub properties: Option<HashMap<String, Schema>>,
    pub required: Vec<String>,
    pub ref_path: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub commit_hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub conventional_type: Option<String>,
    pub conventional_scope: Option<String>,
    pub breaking: bool,
    pub references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub project: ProjectInfo,
    pub modules: IndexMap<String, Module>,
    pub root_module: String,
    pub api_endpoints: Vec<ApiEndpoint>,
    pub schemas: HashMap<String, Schema>,
    pub cross_references: Vec<CrossReference>,
    pub statistics: AnalysisStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub from: String,
    pub to: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_modules: usize,
    pub total_items: usize,
    pub total_endpoints: usize,
    pub languages: HashMap<String, usize>,
    pub doc_coverage: f64,
}
