use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortVersion {
    Alphanumeric,
    V4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub tailwind_attributes: Vec<String>,
    pub tailwind_functions: Vec<String>,

    pub enable_sort: bool,
    // ignore when `enable_sort` is false
    pub sort_version: SortVersion,

    pub enable_wrap: bool,
    // ignore when `enable_wrap` is false
    pub allow_line_overflow: bool,
    // ignore when `enable_wrap` is false
    pub indent_to_quote: bool,
    // ignore when `enable_wrap` is false
    pub indent_width: u8,
    // ignore when `enable_wrap` is false
    pub line_width_includes_indent: bool,
    // ignore when `enable_wrap` & `line_width_relative_to_indent` are false
    pub line_width: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            tailwind_attributes: Vec::new(),
            tailwind_functions: Vec::new(),
            enable_sort: true,
            sort_version: SortVersion::V4,
            enable_wrap: true,
            allow_line_overflow: false,
            indent_to_quote: true,
            indent_width: 2,
            line_width_includes_indent: false,
            line_width: 120,
        }
    }
}

impl Configuration {
    pub fn with_tailwind_attributes(mut self, patterns: Vec<String>) -> Self {
        self.tailwind_attributes = patterns;
        self
    }

    pub fn with_tailwind_functions(mut self, patterns: Vec<String>) -> Self {
        self.tailwind_functions = patterns;
        self
    }

    pub fn with_enable_sort(mut self, enabled: bool) -> Self {
        self.enable_wrap = enabled;
        self
    }

    pub fn with_sort_version(mut self, version: SortVersion) -> Self {
        self.sort_version = version;
        self
    }

    pub fn with_enable_wrap(mut self, enabled: bool) -> Self {
        self.enable_wrap = enabled;
        self
    }

    pub fn with_allow_line_overflow(mut self, enabled: bool) -> Self {
        self.allow_line_overflow = enabled;
        self
    }

    pub fn with_indent_to_quote(mut self, value: bool) -> Self {
        self.indent_to_quote = value;
        self
    }

    pub fn with_indent_width(mut self, width: u8) -> Self {
        self.indent_width = width;
        self
    }

    pub fn with_line_width_includes_indent(mut self, enabled: bool) -> Self {
        self.line_width_includes_indent = enabled;
        self
    }

    pub fn with_line_width(mut self, width: u32) -> Self {
        self.line_width = width;
        self
    }
}
