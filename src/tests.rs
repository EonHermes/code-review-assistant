#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::types::AnalysisStats;
    use std::path::PathBuf;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(!config.exclude.is_empty());
        assert!(!config.include.is_empty());
        assert!(config.exclude.contains("target"));
        assert!(config.exclude.contains("node_modules"));
        assert!(config.exclude.contains(".git"));
    }

    #[test]
    fn test_config_rust_settings() {
        let config = Config::default();
        assert!(config.language.rust.analyze_macros);
        assert!(config.language.rust.include_docs_comments);
        assert!(!config.language.rust.extract_tests);
    }

    #[test]
    fn test_config_api_detection() {
        let config = Config::default();
        assert!(config.api_detection.auto);
        assert!(config.api_detection.frameworks.contains(&"actix-web".to_string()));
        assert!(config.api_detection.frameworks.contains(&"rocket".to_string()));
    }

    #[test]
    fn test_graph_config() {
        let config = Config::default();
        assert_eq!(config.graph.format, "svg");
        assert!(!config.graph.include_external);
        assert!(config.graph.cluster_modules);
    }

    #[test]
    fn test_changelog_config() {
        let config = Config::default();
        assert!(config.changelog.conventional_commits);
        assert!(config.changelog.include_links);
        assert!(config.changelog.max_entries.is_none());
    }

    #[test]
    fn test_analysis_stats_creation() {
        let stats = AnalysisStats {
            total_files: 10,
            total_lines: 1000,
            total_modules: 5,
            total_items: 50,
            total_endpoints: 3,
            languages: {
                let mut map = std::collections::HashMap::new();
                map.insert("Rust".to_string(), 10);
                map.insert("Markdown".to_string(), 2);
                map
            },
            doc_coverage: 85.5,
        };

        assert_eq!(stats.total_files, 10);
        assert_eq!(stats.total_modules, 5);
        assert_eq!(stats.doc_coverage, 85.5);
        assert_eq!(stats.languages.get(&"Rust".to_string()), Some(&10));
    }

    #[test]
    fn test_config_load_defaults() {
        let temp_dir = PathBuf::from("/tmp");
        let config = Config::load(&temp_dir, None).unwrap();
        // Should have defaults
        assert!(config.exclude.contains("target"));
        assert!(config.include.contains("src"));
    }

    #[test]
    fn test_config_custom_exclude() {
        let temp_dir = PathBuf::from("/tmp");
        // Create a custom config file
        let config_content = r#"
exclude:
  - custom_dir
  - another_dir
include:
  - src
  - lib
"#;
        let config_path = temp_dir.join("test_config.yaml");
        std::fs::write(&config_path, config_content).unwrap();

        let config = Config::load(&temp_dir, Some(&config_path)).unwrap();
        assert!(config.exclude.contains("custom_dir"));
        assert!(config.exclude.contains("another_dir"));
        assert!(config.include.contains("src"));
        assert!(config.include.contains("lib"));

        // Cleanup
        std::fs::remove_file(config_path).unwrap();
    }
}
