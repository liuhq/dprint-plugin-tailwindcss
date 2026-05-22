use std::collections::{BTreeMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CssConfig {
    pub custom_utilities: HashSet<String>,
    pub breakpoints: BTreeMap<String, usize>,
    pub custom_variants: HashSet<String>,
}

impl CssConfig {
    pub fn breakpoint_order(&self, name: &str) -> usize {
        self.breakpoints.get(name).copied().unwrap_or(usize::MAX)
    }
}

impl Default for CssConfig {
    fn default() -> Self {
        let mut breakpoints = BTreeMap::new();
        let screens = ["sm", "md", "lg", "xl", "2xl"];
        for (i, name) in screens.iter().enumerate() {
            breakpoints.insert(name.to_string(), i);
        }
        CssConfig {
            custom_utilities: HashSet::new(),
            breakpoints,
            custom_variants: HashSet::new(),
        }
    }
}

pub fn parse_css_config(css_text: &str) -> anyhow::Result<CssConfig> {
    let mut config = CssConfig::default();
    let mut in_theme = false;
    let mut theme_brace_depth = 0u32;
    let mut breakpoint_order = 0usize;

    for line in css_text.lines() {
        let trimmed = line.trim();

        if in_theme {
            for (_end_idx, _) in trimmed.match_indices('}') {
                theme_brace_depth = theme_brace_depth.saturating_sub(1);
                if theme_brace_depth == 0 {
                    in_theme = false;
                    break;
                }
            }
            if !in_theme {
                continue;
            }
            for (_start_idx, _) in trimmed.match_indices('{') {
                theme_brace_depth += 1;
            }

            if let Some(bp_name) = extract_breakpoint(trimmed) {
                config.breakpoints.entry(bp_name).or_insert_with(|| {
                    let idx = breakpoint_order;
                    breakpoint_order += 1;
                    idx
                });
            }
            continue;
        }

        if let Some(util_name) = extract_custom_utility(trimmed) {
            config.custom_utilities.insert(util_name);
            continue;
        }

        if let Some(variant_name) = extract_custom_variant(trimmed) {
            config.custom_variants.insert(variant_name);
            continue;
        }

        if let Some(start_pos) = trimmed.find("@theme") {
            let after_theme = &trimmed[start_pos + "@theme".len()..];
            in_theme = true;
            theme_brace_depth = 0;
            if after_theme.trim().starts_with('{') {
                theme_brace_depth = 1;
            }
            continue;
        }
    }

    Ok(config)
}

fn extract_custom_utility(line: &str) -> Option<String> {
    if !line.starts_with("@utility") {
        return None;
    }
    let rest = &line["@utility".len()..].trim();
    let name_end = rest
        .find(|c: char| c == '{' || c.is_whitespace())
        .unwrap_or(rest.len());
    let name = rest[..name_end].trim().trim_matches('"').trim_matches('\'').trim_matches('`');
    if name.is_empty() {
        return None;
    }
    Some(name.to_string())
}

fn extract_custom_variant(line: &str) -> Option<String> {
    if !line.starts_with("@custom-variant") {
        return None;
    }
    let rest = &line["@custom-variant".len()..].trim();
    let name_end = rest
        .find(|c: char| c == '(' || c == '{' || c.is_whitespace())
        .unwrap_or(rest.len());
    let name = rest[..name_end].trim().trim_matches('"').trim_matches('\'').trim_matches('`');
    if name.is_empty() {
        return None;
    }
    Some(name.to_string())
}

fn extract_breakpoint(line: &str) -> Option<String> {
    let line = line.trim();
    if !line.starts_with("--breakpoint-") {
        return None;
    }
    let rest = &line["--breakpoint-".len()..];
    let value_sep = rest.find(':').unwrap_or(rest.len());
    let name = rest[..value_sep].trim();
    if name.is_empty() {
        return None;
    }
    Some(name.to_string())
}

pub fn find_css_file(base_dir: &Path, configured_path: Option<&str>) -> Option<String> {
    if let Some(path) = configured_path {
        let full_path = base_dir.join(path);
        if full_path.exists() {
            return Some(full_path.to_string_lossy().to_string());
        }
    }
    None
}
