use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use dprint_core::configuration::{ConfigurationDiagnostic, GlobalConfiguration, NewLineKind};
pub use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename_all = "camelCase")]
pub struct Configuration {
    /// The width of a line the printer will try to stay under. Note that the printer may exceed this width in some cases.
    #[schemars(default = "default_line_width")]
    pub line_width: u32,
    /// The number of characters for an indent.
    #[schemars(default = "default_indent_width")]
    pub indent_width: u8,
    /// Whether to use tabs (true) or spaces (false).
    #[schemars(default = "default_use_tabs")]
    pub use_tabs: bool,
    /// The kind of newline to use.
    #[schemars(default = "default_new_line_kind", with = "String")]
    pub new_line_kind: NewLineKind,
}

fn default_line_width() -> u32 {
    80
}
fn default_indent_width() -> u8 {
    2
}
fn default_use_tabs() -> bool {
    false
}
fn default_new_line_kind() -> NewLineKind {
    NewLineKind::LineFeed
}

pub fn resolve_config(
    config: serde_json::Value,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();
    let mut config = config;

    let resolved_config = Configuration {
        line_width: get_value(
            &mut config,
            "lineWidth",
            global_config.line_width.unwrap_or(80),
            &mut diagnostics,
        ),
        indent_width: get_value(
            &mut config,
            "indentWidth",
            global_config.indent_width.unwrap_or(2),
            &mut diagnostics,
        ),
        use_tabs: get_value(
            &mut config,
            "useTabs",
            global_config.use_tabs.unwrap_or(false),
            &mut diagnostics,
        ),
        new_line_kind: get_value(
            &mut config,
            "newLineKind",
            global_config.new_line_kind.unwrap_or(NewLineKind::LineFeed),
            &mut diagnostics,
        ),
    };

    PluginResolveConfigurationResult {
        config: resolved_config,
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec!["adoc".to_string(), "asciidoc".to_string()],
            file_names: vec![],
        },
    }
}

fn get_value<T>(
    config: &mut serde_json::Value,
    key: &str,
    default_value: T,
    diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> T
where
    for<'de> T: Deserialize<'de>,
{
    if let Some(value) = config.as_object_mut().and_then(|o| o.remove(key)) {
        match serde_json::from_value::<T>(value) {
            Ok(v) => v,
            Err(err) => {
                diagnostics.push(ConfigurationDiagnostic {
                    property_name: key.to_string(),
                    message: err.to_string(),
                });
                default_value
            }
        }
    } else {
        default_value
    }
}
