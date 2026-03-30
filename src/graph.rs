use crate::types::{AnalysisResult, CrossReference};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use std::collections::{HashMap, HashSet};

pub struct DependencyGraph<'a> {
    result: &'a AnalysisResult,
}

impl<'a> DependencyGraph<'a> {
    pub fn new(result: &'a AnalysisResult) -> Self {
        Self { result }
    }

    pub fn compute_references(&self) -> Vec<CrossReference> {
        let mut references = Vec::new();
        let mut graph = DiGraph::<String, ()>::new();
        let mut indices: HashMap<String, NodeIndex> = HashMap::new();

        // Add module nodes
        for (module_id, _) in &self.result.modules {
            let idx = graph.add_node(module_id.clone());
            indices.insert(module_id.clone(), idx);
        }

        // Add edges based on module hierarchy
        for (module_id, module) in &self.result.modules {
            if let Some(parent_id) = &module.parent {
                if let (Some(&parent_idx), Some(&child_idx)) = (indices.get(parent_id), indices.get(module_id)) {
                    graph.add_edge(parent_idx, child_idx, ());
                    references.push(CrossReference {
                        from: parent_id.clone(),
                        to: module_id.clone(),
                        kind: "contains".to_string(),
                    });
                }
            }
        }

        // Add item references within modules
        for (module_id, module) in &self.result.modules {
            let module_idx = indices.get(module_id);
            for item in &module.items {
                // Parse signature to find referenced types (simplified)
                if let Some(signature) = &item.signature {
                    for (ref_mod_id, _) in &self.result.modules {
                        if signature.contains(ref_mod_id.split("::").last().unwrap_or(&ref_mod_id)) {
                            if let (Some(&from_idx), Some(&to_idx)) = (module_idx, indices.get(ref_mod_id)) {
                                if from_idx != to_idx {
                                    graph.add_edge(from_idx, to_idx, ());
                                    references.push(CrossReference {
                                        from: module_id.clone(),
                                        to: ref_mod_id.clone(),
                                        kind: "uses".to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        references
    }

    pub fn to_dot(&self) -> Result<String, Box<dyn std::error::Error>> {
        use dot::Graph;

        let mut g = Graph::new();
        let mut indices: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

        // Add nodes
        let mut root_subgraph = g.add_subgraph(vec![], Some("cluster_root".to_string()));
        for (module_id, module) in &self.result.modules {
            let node = g.add_node(format!("{} ({})\\n{} items", 
                module.name, 
                module_id,
                module.items.len()
            ));
            indices.insert(module_id.clone(), node);

            // Add to appropriate subgraph (clustering by top-level module)
            if let Some(parent) = &module.parent {
                if parent == "root" || !parent.contains("::") {
                    root_subgraph.push(node);
                } else {
                    // Could create nested clusters here
                    root_subgraph.push(node);
                }
            } else {
                root_subgraph.push(node);
            }
        }

        // Add edges
        for (module_id, module) in &self.result.modules {
            if let Some(parent_id) = &module.parent {
                if let (Some(&parent_idx), Some(&child_idx)) = (indices.get(parent_id), indices.get(module_id)) {
                    g.add_edge(parent_idx, child_idx, "contains".to_string());
                }
            }
        }

        Ok(g.to_string())
    }
}
