use std::path::Path;

use dprint_development::*;
use dprint_plugin_tailwindcss::configuration::Configuration;
use dprint_plugin_tailwindcss::format_text;
use dprint_plugin_tailwindcss::FormatTextOptions;

fn main() {
    run_specs(
        &Path::new("./tests/specs"),
        &ParseSpecOptions {
            default_file_name: "test.tsx",
        },
        &RunSpecsOptions {
            fix_failures: false,
            format_twice: false,
        },
        std::sync::Arc::new(|file_path, file_text, spec_config| {
            let mut config = Configuration {
                css_file: None,
            };

            if let Some(css) = spec_config.get("cssFile") {
                if let Some(s) = css.as_str() {
                    config.css_file = Some(s.to_string());
                }
            }

            let extension = file_path.extension().and_then(|e| e.to_str());

            format_text(FormatTextOptions {
                path: file_path,
                extension,
                text: file_text,
                config: &config,
            })
        }),
        std::sync::Arc::new(|_file_path, _file_text, _spec_config| String::from("{}")),
    );
}
