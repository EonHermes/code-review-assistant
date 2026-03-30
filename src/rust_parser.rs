use crate::parser_trait::LanguageParser;
use crate::types::{Module, ModuleItem, ItemKind, Visibility, AnalysisResult};
use crate::errors::Result;
use indexmap::IndexMap;
use std::path::Path;
use syn::visit::Visit;
use quote::ToTokens;
use regex::Regex;

pub struct RustParser {
    modules: IndexMap<String, Module>,
}

impl RustParser {
    pub fn new() -> Self {
        Self { modules: IndexMap::new() }
    }
}

impl LanguageParser for RustParser {
    fn parse_project(&mut self, root: &Path, result: &mut AnalysisResult) -> Result<()> {
        self.discover_modules(root)?;
        self.parse_all_files(root)?;
        result.modules = self.modules.clone();
        result.root_module = "root".to_string();
        self.detect_api_endpoints(result)?;
        Ok(())
    }
}

impl RustParser {
    fn discover_modules(&mut self, root: &Path) -> Result<()> {
        let src_dir = root.join("src");
        if !src_dir.exists() { return Ok(()); }

        for entry in walkdir::WalkDir::new(&src_dir).into_iter().filter(|e| e.is_ok()).map(|e| e.unwrap()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let relative = path.strip_prefix(&src_dir).unwrap_or(path);
                let mod_name = relative.with_extension("")
                    .to_string_lossy()
                    .replace("/", "::")
                    .replace("\\", "::");
                let module_id = mod_name.clone();

                if let Ok(content) = std::fs::read_to_string(path) {
                    let mut module = Module {
                        id: module_id.clone(),
                        name: mod_name.split("::").last().unwrap_or(&module_id).to_string(),
                        path: relative.to_string_lossy().to_string(),
                        parent: None,
                        children: Vec::new(),
                        items: Vec::new(),
                        documentation: self.extract_file_docs(&content),
                        line_count: content.lines().count(),
                    };

                    if let Ok(re) = Regex::new(r"mod\s+(\w+)\s*;") {
                        for cap in re.captures_iter(&content) {
                            if let Some(submod) = cap.get(1) {
                                let submod_id = format!("{}::{}", module_id, submod.as_str());
                                module.children.push(submod_id);
                            }
                        }
                    }

                    self.modules.insert(module_id.clone(), module);
                }
            }
        }

        for (id, module) in self.modules.iter_mut() {
            if let Some((parent, _)) = id.rsplit_once("::") {
                let parent_id = parent.to_string();
                if self.modules.contains_key(&parent_id) {
                    module.parent = Some(parent_id);
                }
            }
        }

        Ok(())
    }

    fn parse_all_files(&mut self, root: &Path) -> Result<()> {
        let src_dir = root.join("src");

        for (module_id, module) in self.modules.clone().iter() {
            let mut file_path = src_dir.join(&module.path);
            if !file_path.exists() {
                file_path = src_dir.join(format!("{}.rs", &module.path));
            }
            if !file_path.exists() {
                let mut mod_path = src_dir.join(&module.path);
                mod_path.set_file_name("mod.rs");
                if mod_path.exists() {
                    file_path = mod_path;
                }
            }

            if file_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    self.parse_file_content(module_id, &content)?;
                }
            }
        }

        Ok(())
    }

    fn parse_file_content(&mut self, module_id: &str, content: &str) -> Result<()> {
        let syntax = syn::parse_file(content)
            .map_err(|e| crate::errors::AnalysisError::Parse(format!("{}: {}", module_id, e)))?;

        let mut visitor = RustItemVisitor {
            module_id: module_id.to_string(),
            items: Vec::new(),
        };

        visitor.visit_file(&syntax);

        if let Some(module) = self.modules.get_mut(module_id) {
            module.items = visitor.items;
        }

        Ok(())
    }

    fn detect_api_endpoints(&self, result: &mut AnalysisResult) -> Result<()> {
        let route_re = Regex::new(r#"\[(route|get|post|put|delete|patch)\((.*?)\)\]"#).unwrap();
        let path_re = Regex::new(r#""([^"]+)""#).unwrap();

        for module in result.modules.values() {
            for item in &module.items {
                if item.kind == ItemKind::Function && item.attributes.iter().any(|a| a.contains("route")) {
                    if let Some(path_match) = route_re.find(&item.attributes.join(" ")) {
                        let mut method = "GET".to_string();
                        if let Some(m) = Regex::new(r"(get|post|put|delete|patch)").unwrap()
                            .find(&path_match.as_str())
                        {
                            method = m.as_str().to_uppercase();
                        }

                        let path = path_re
                            .find(path_match.as_str())
                            .map(|m| m.as_str().trim_matches('"'))
                            .unwrap_or("/")
                            .to_string();

                        result.api_endpoints.push(crate::types::ApiEndpoint {
                            path,
                            method,
                            summary: item.documentation.clone(),
                            description: None,
                            parameters: Vec::new(),
                            request_body: None,
                            responses: Vec::new(),
                            tags: Vec::new(),
                            source_file: module.path.clone(),
                            line: item.line,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn extract_file_docs(&self, content: &str) -> Option<String> {
        for line in content.lines().take(20) {
            if line.trim().starts_with("//!") {
                let doc = line.trim_start_matches("//! ").trim().to_string();
                if !doc.is_empty() { return Some(doc); }
            }
        }

        if let Ok(re) = Regex::new(r"(?s)/\*\*\s*([^*]*\*+[^*]*\*+)(\n|\r\n|\r)") {
            if let Some(cap) = re.find(content) {
                let doc = cap.as_str();
                let clean = doc[3..doc.len()-2].replace("**/", "").trim().to_string();
                if !clean.is_empty() { return Some(clean); }
            }
        }

        None
    }
}

struct RustItemVisitor {
    module_id: String,
    items: Vec<ModuleItem>,
}

impl<'ast> syn::visit::Visit<'ast> for RustItemVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::Function,
            &i.vis,
            Some(format!("{}", i.sig.to_token_stream())),
            &i.attrs,
            i.span().start().line,
        ));
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::Struct,
            &i.vis,
            Some(format!("{} struct {}", visibility_str(&i.vis), i.ident)),
            &i.attrs,
            i.span().start().line,
        ));
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::Enum,
            &i.vis,
            Some(format!("{} enum {}", visibility_str(&i.vis), i.ident)),
            &i.attrs,
            i.span().start().line,
        ));
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::Trait,
            &i.vis,
            Some(format!("{} trait {}", visibility_str(&i.vis), i.ident)),
            &i.attrs,
            i.span().start().line,
        ));
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::Constant,
            &i.vis,
            Some(format!("{} const {}: {}", visibility_str(&i.vis), i.ident, i.ty.to_token_stream())),
            &i.attrs,
            i.span().start().line,
        ));
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::TypeAlias,
            &i.vis,
            Some(format!("{} type {} = {}", visibility_str(&i.vis), i.ident, i.ty.to_token_stream())),
            &i.attrs,
            i.span().start().line,
        ));
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        self.items.push(self.create_item(
            i.ident.to_string(),
            ItemKind::Module,
            &i.vis,
            Some(format!("{} mod {}", visibility_str(&i.vis), i.ident)),
            &i.attrs,
            i.span().start().line,
        ));
    }
}

impl RustItemVisitor {
    fn create_item(
        &self,
        name: String,
        kind: ItemKind,
        vis: &syn::Visibility,
        signature: Option<String>,
        attrs: &[syn::Attribute],
        line: u32,
    ) -> ModuleItem {
        let visibility = match vis {
            syn::Visibility::Public(_) => Visibility::Public,
            syn::Visibility::Restricted(restricted) => {
                if let Some(syn::VisRestricted { path, .. }) = restricted {
                    if path.is_ident("crate") {
                        Visibility::Restricted("crate".to_string())
                    } else if path.is_ident("self") {
                        Visibility::Restricted("self".to_string())
                    } else if path.is_ident("super") {
                        Visibility::Restricted("super".to_string())
                    } else {
                        Visibility::Restricted(path.to_token_stream().to_string())
                    }
                } else {
                    Visibility::Private
                }
            }
            _ => Visibility::Private,
        };

        let attributes = attrs.iter().map(|a| a.to_string()).collect();
        let documentation = self.extract_doc_comment(attrs);

        ModuleItem {
            id: format!("{}::{}", self.module_id, name),
            name,
            kind,
            visibility,
            signature,
            documentation,
            attributes,
            line: line as usize,
            children: Vec::new(),
        }
    }

    fn extract_doc_comment(&self, attrs: &[syn::Attribute]) -> Option<String> {
        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &nv.lit {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value().trim().to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

fn visibility_str(vis: &syn::Visibility) -> &'static str {
    match vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Restricted(_) => "pub",
        _ => "",
    }
}
