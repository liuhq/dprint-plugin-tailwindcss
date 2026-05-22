use dprint_core::configuration::*;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::*;

use super::configuration::Configuration;
use super::format_text::{FormatTextOptions, format_text};

struct TailwindCssPluginHandler;

impl SyncPluginHandler<Configuration> for TailwindCssPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let ResolveConfigurationResult {
            config,
            diagnostics,
        } = Configuration::resolve_config(config, global_config);

        PluginResolveConfigurationResult {
            config,
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec![
                    String::from("tsx"),
                    String::from("jsx"),
                    String::from("html"),
                ],
                file_names: Vec::new(),
            },
        }
    }

    fn plugin_info(&mut self) -> PluginInfo {
        let name = env!("CARGO_PKG_NAME").to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name,
            version: version.clone(),
            config_key: "tailwindcss".to_string(),
            help_url: "".to_string(),
            config_schema_url: "".to_string(),
            update_url: Some(
                "https://plugins.dprint.dev/liuhq/dprint-plugin-tailwindcss/latest.json"
                    .to_string(),
            ),
        }
    }

    fn license_text(&mut self) -> String {
        // std::str::from_utf8(include_bytes!("../LICENSE"))
        //     .unwrap()
        //     .into()
        String::new()
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> anyhow::Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        let file_text = String::from_utf8(request.file_bytes)?;
        let file_text_str = &file_text;
        format_text(FormatTextOptions {
            path: request.file_path,
            extension: None,
            text: file_text_str,
            config: request.config,
        })
        .map(|maybe_text| maybe_text.map(|t| t.into_bytes()))
    }
}

generate_plugin_code!(TailwindCssPluginHandler, TailwindCssPluginHandler);
