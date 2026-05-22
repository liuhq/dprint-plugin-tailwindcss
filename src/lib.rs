pub mod configuration;
pub mod class_finder;
pub mod class_sorter;
pub mod css_config;
pub mod format_text;
pub mod parsed_class;
pub mod sort_order;

pub use format_text::FormatTextOptions;
pub use format_text::format_text;

#[cfg(feature = "wasm")]
#[cfg(target_arch = "wasm32")]
#[cfg(target_os = "unknown")]
mod wasm_plugin;
#[cfg(feature = "wasm")]
#[cfg(target_arch = "wasm32")]
#[cfg(target_os = "unknown")]
pub use wasm_plugin::*;