use std::path::Path;

use crate::class_finder::find_classes;
use crate::class_sorter::sort_class_string;
use crate::configuration::Configuration;
use crate::css_config::{CssConfig, parse_css_config};

pub struct FormatTextOptions<'a> {
    pub path: &'a Path,
    pub extension: Option<&'a str>,
    pub text: &'a str,
    pub config: &'a Configuration,
}

pub fn format_text(options: FormatTextOptions<'_>) -> anyhow::Result<Option<String>> {
    let extension = options
        .extension
        .or_else(|| options.path.extension().and_then(|e| e.to_str()))
        .unwrap_or("")
        .to_lowercase();

    let css_config = if let Some(ref css_file) = options.config.css_file {
        let base_dir = options.path.parent().unwrap_or(Path::new("."));
        let full_path = base_dir.join(css_file);
        if full_path.exists() {
            let css_text = std::fs::read_to_string(&full_path)?;
            parse_css_config(&css_text)?
        } else {
            CssConfig::default()
        }
    } else {
        CssConfig::default()
    };

    let class_spans = find_classes(options.text, &extension)?;

    let mut replacements: Vec<(usize, usize, String)> = Vec::new();

    for span in &class_spans {
        if let Some(sorted) = sort_class_string(&span.classes, &css_config) {
            replacements.push((span.start, span.end, sorted));
        }
    }

    if replacements.is_empty() {
        return Ok(None);
    }

    replacements.sort_by_key(|b| std::cmp::Reverse(b.0));

    let mut result = options.text.to_string();
    for (start, end, sorted) in &replacements {
        result.replace_range(*start..*end, sorted);
    }

    Ok(Some(result))
}
