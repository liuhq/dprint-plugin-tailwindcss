use std::usize;

use dprint_core::formatting::{PrintItems, Signal, ir_helpers, utils::string_utils};
use dprint_core_macros::sc;

use crate::generation::types::{IntoU32, IntoUsize};

enum IndentCount {
    IndentToQuote(u32),
    IndentToPre(u32),
}

impl IndentCount {
    fn value(&self) -> u32 {
        match self {
            IndentCount::IndentToQuote(n) | IndentCount::IndentToPre(n) => *n,
        }
    }

    fn level(&self, indent_width: u8) -> u32 {
        get_indent_level(self.value().into_usize(), indent_width)
    }
}

pub struct TailwindWrapperOption {
    pub allow_line_overflow: bool,
    pub indent_to_quote: bool,
    pub indent_width: u8,
    pub line_width_includes_indent: bool,
    pub line_width: u32,
}

pub struct TailwindWrapper {
    option: TailwindWrapperOption,
    pre_jsx_element_line: u32,
    pre_indent_count: u32,
    jsxexpression: bool,
}

impl TailwindWrapper {
    pub fn new(option: TailwindWrapperOption) -> Self {
        Self {
            option,
            pre_jsx_element_line: 0,
            pre_indent_count: 0,
            jsxexpression: false,
        }
    }

    pub fn set_pre_jsx_element_line(&mut self, source_text: &str, node_span_start: usize) {
        self.pre_jsx_element_line = get_line_number(source_text, node_span_start);
    }

    pub fn set_pre_indent_count(&mut self, source_text: &str, node_span_start: usize) {
        self.pre_indent_count = get_column_number(source_text, node_span_start)
    }

    pub fn enter_jsxexpression(&mut self) {
        self.jsxexpression = true
    }

    pub fn leave_jsxexpression(&mut self) {
        self.jsxexpression = false
    }
}

impl TailwindWrapper {
    pub fn format(
        &self,
        node_text: &str,
        source_text: &str,
        attr_name_span_start: usize,
        attr_value_span_start: usize,
    ) -> PrintItems {
        let indent_count =
            self.parse_indent(source_text, attr_name_span_start, attr_value_span_start);

        let wrapped_items = self.wrap_text(
            node_text,
            &indent_count,
            get_column_number(source_text, attr_value_span_start),
        );

        if self.option.indent_to_quote {
            return wrapped_items;
        }

        ir_helpers::with_indent_times(wrapped_items, indent_count.level(self.option.indent_width))
    }
}

impl TailwindWrapper {
    fn parse_indent(
        &self,
        source_text: &str,
        attr_name_span_start: usize,
        attr_value_span_start: usize,
    ) -> IndentCount {
        if self.option.indent_to_quote {
            IndentCount::IndentToQuote(get_column_number(source_text, attr_value_span_start))
        } else {
            let line = get_line_number(source_text, attr_value_span_start);
            let indent_count = if line == self.pre_jsx_element_line {
                self.pre_indent_count + self.option.indent_width.into_u32()
            } else {
                get_column_number(source_text, attr_name_span_start)
                    + self.option.indent_width.into_u32()
            };

            IndentCount::IndentToPre(indent_count)
        }
    }

    fn wrap_text(
        &self,
        node_text: &str,
        indent_count: &IndentCount,
        first_lint_column: u32,
    ) -> PrintItems {
        let push_break_line = |items: &mut PrintItems| {
            push_jsxexpression_endl(items, self.jsxexpression);
            push_newline(items);
            if let IndentCount::IndentToQuote(column) = indent_count {
                push_spaces(items, *column);
            }
        };

        let mut current_width = first_lint_column;

        let parts: Vec<_> = node_text.split_whitespace().collect();
        let last_index = parts.len() - 1;

        parts
            .iter()
            .enumerate()
            .fold(PrintItems::new(), |mut items, (i, text)| {
                let text_width = text.chars().count().into_u32();
                let next_width = current_width + text_width + 1;
                let exceeds = if self.option.line_width_includes_indent {
                    next_width > self.option.line_width - indent_count.value()
                } else {
                    next_width > self.option.line_width
                };
                let not_first = i > 0;

                match (exceeds, self.option.allow_line_overflow) {
                    (true, true) => {
                        push_text(&mut items, text, current_width > 0);
                        current_width = 0;
                        if i < last_index {
                            push_break_line(&mut items);
                        }
                    }
                    (true, false) => {
                        push_break_line(&mut items);
                        push_text(&mut items, text, false);
                        current_width = text_width;
                    }
                    _ => {
                        push_text(&mut items, text, not_first);
                        current_width = next_width;
                    }
                }

                items
            })
    }
}

/// 0-indexed
fn get_column_number(text: &str, start: usize) -> u32 {
    string_utils::get_column_number_of_pos(text, start).into_u32() - 1
}

/// 0-indexed
fn get_line_number(text: &str, start: usize) -> u32 {
    string_utils::get_line_number_of_pos(text, start).into_u32() - 1
}

/// 0-indexed
fn get_indent_level(column: usize, indent_width: u8) -> u32 {
    let indent_width = indent_width.into_u32();
    column.into_u32() / indent_width
}

fn push_spaces(items: &mut PrintItems, space_count: u32) {
    (0..space_count).for_each(|_| items.push_space());
}

fn push_text(items: &mut PrintItems, text: &str, leading_space: bool) {
    if leading_space {
        push_spaces(items, 1);
    }
    items.push_string(text.to_string());
}

fn push_jsxexpression_endl(items: &mut PrintItems, jsxexpression: bool) {
    if jsxexpression {
        items.push_space();
        items.push_sc(sc!("\\"));
    }
}

fn push_newline(items: &mut PrintItems) {
    items.push_signal(Signal::NewLine);
}
