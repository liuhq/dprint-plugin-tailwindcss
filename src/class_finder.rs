use deno_ast::view::{
    JSXAttrOrSpread, JSXAttrValue, JSXElement, NodeTrait,
};
use deno_ast::{parse_module, MediaType, ParseParams, SourceRanged};
use std::sync::Arc;

pub struct ClassSpan {
    pub start: usize,
    pub end: usize,
    pub classes: String,
}

pub fn find_classes(text: &str, extension: &str) -> anyhow::Result<Vec<ClassSpan>> {
    match extension {
        "tsx" | "jsx" => find_classes_jsx(text, extension),
        "html" => Ok(find_classes_html(text)),
        _ => Ok(Vec::new()),
    }
}

fn find_classes_jsx(text: &str, extension: &str) -> anyhow::Result<Vec<ClassSpan>> {
    let media_type = match extension {
        "tsx" => MediaType::Tsx,
        _ => MediaType::Jsx,
    };

    let specifier = deno_ast::ModuleSpecifier::parse("file:///input.tsx").unwrap();
    let parsed = parse_module(ParseParams {
        specifier,
        text: Arc::from(text),
        media_type,
        capture_tokens: true,
        scope_analysis: false,
        maybe_syntax: None,
    })?;

    let mut spans = Vec::new();
    let root_start = parsed.range().start;

    parsed.with_view(|program| {
        let root = program.as_node();
        walk_jsx_node(&root, root_start, text, &mut spans);
    });

    Ok(spans)
}

fn walk_jsx_node(
    node: &deno_ast::view::Node,
    root_start: deno_ast::StartSourcePos,
    source_text: &str,
    spans: &mut Vec<ClassSpan>,
) {
    if let Some(element) = node.to::<JSXElement>() {
        let opening = &element.opening;
        for attr in opening.attrs.iter() {
            if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
                let name_str = match &jsx_attr.name {
                    deno_ast::view::JSXAttrName::Ident(ident) => {
                        ident.inner.sym.to_string()
                    }
                    _ => continue,
                };
                if (name_str == "className" || name_str == "class")
                    && let Some(value) = &jsx_attr.value {
                        process_attr_value(value, root_start, source_text, spans);
                    }
            }
        }
    }

    let children = node.children();
    for child in children.iter() {
        walk_jsx_node(child, root_start, source_text, spans);
    }
}

fn process_attr_value(
    value: &JSXAttrValue,
    root_start: deno_ast::StartSourcePos,
    source_text: &str,
    spans: &mut Vec<ClassSpan>,
) {
    match value {
        JSXAttrValue::Str(s) => {
            let range = s.range();
            let byte_range = range.as_byte_range(root_start);
            let inner_val = source_text[byte_range.start + 1..byte_range.end - 1].to_string();
            spans.push(ClassSpan {
                start: byte_range.start + 1,
                end: byte_range.end - 1,
                classes: inner_val,
            });
        }
        JSXAttrValue::JSXExprContainer(container) => {
            let range = container.range();
            let byte_range = range.as_byte_range(root_start);
            if byte_range.start + 2 >= byte_range.end {
                return;
            }
            let inner_text = &source_text[byte_range.start..byte_range.end];
            if inner_text.starts_with('{') && inner_text.ends_with('}') {
                let inner = inner_text[1..inner_text.len() - 1].trim();
                if inner.starts_with('`') && inner.ends_with('`') {
                    let tpl_text = &inner[1..inner.len() - 1];
                    spans.push(ClassSpan {
                        start: byte_range.start + 2,
                        end: byte_range.end - 2,
                        classes: tpl_text.to_string(),
                    });
                }
            }
        }
        _ => {}
    }
}

fn find_classes_html(text: &str) -> Vec<ClassSpan> {
    let mut spans = Vec::new();
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let rest = &text[i..];

        let class_pos = match rest.find("class=") {
            Some(p) => p,
            None => break,
        };

        i += class_pos;

        if i + 7 > len {
            break;
        }

        let attr_start = i + 6;
        if attr_start >= len {
            i += 6;
            continue;
        }

        let quote = bytes[attr_start];
        if quote != b'"' && quote != b'\'' {
            i = attr_start;
            continue;
        }

        let value_start = attr_start + 1;
        let mut pos = value_start;
        while pos < len && bytes[pos] != quote {
            pos += 1;
        }
        let value_end = pos;

        if value_start < value_end {
            let classes = std::str::from_utf8(&bytes[value_start..value_end])
                .unwrap_or("")
                .to_string();
            if !classes.is_empty() {
                spans.push(ClassSpan {
                    start: value_start,
                    end: value_end,
                    classes,
                });
            }
        }

        if pos + 1 < len && bytes[pos] == quote {
            i = pos + 2;
        } else {
            i = pos + 1;
        }
    }

    spans
}
