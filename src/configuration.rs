use dprint_core::configuration::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub css_file: Option<String>,
}

impl Configuration {
    pub fn resolve_config(
        mut config: ConfigKeyMap,
        _global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Self> {
        let mut diagnostics = Vec::new();

        let css_file = get_nullable_value::<String>(&mut config, "cssFile", &mut diagnostics);

        diagnostics.extend(get_unknown_property_diagnostics(config));

        ResolveConfigurationResult {
            config: Configuration { css_file },
            diagnostics,
        }
    }
}
