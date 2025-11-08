use dprint_core::formatting::PrintItems;
use oxc::{allocator::Allocator, ast_visit::Visit, parser::Parser, span::SourceType};
use std::path::Path;

use crate::{
    configuration::Configuration,
    generation::{
        tw_visitor::TailwindVisitor,
        tw_wrapper::{TailwindWrapper, TailwindWrapperOption},
    },
};

pub fn generate(
    path: &Path,
    source_text: &str,
    config: &Configuration,
) -> anyhow::Result<PrintItems> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).unwrap();
    let parsed = Parser::new(&allocator, source_text, source_type).parse();
    let program = parsed.program;
    let mut visitor = TailwindVisitor::new(source_text, config)
        .with_sorter(None)
        .with_wrapper(if config.enable_wrap {
            Some(TailwindWrapper::new(TailwindWrapperOption {
                allow_line_overflow: config.allow_line_overflow,
                indent_to_quote: config.indent_to_quote,
                indent_width: config.indent_width,
                line_width_includes_indent: config.line_width_includes_indent,
                line_width: config.line_width,
            }))
        } else {
            None
        });
    visitor.visit_program(&program);

    Ok(visitor.print_items())
}
