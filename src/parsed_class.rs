use std::fmt;

pub const SEPARATOR: &str = ":";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VariantLevel {
    None,
    State,
    Screen,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SideOrder {
    Shorthand,
    Axis,
    Direction,
}

#[derive(Debug, Clone)]
pub enum Variant {
    Screen(String),
    State(String),
    ArbitrarySelector(String),
    ArbitraryMedia(String),
    Direction,
}

#[derive(Debug, Clone)]
pub struct ParsedClass {
    pub variants: Vec<Variant>,
    pub importance: bool,
    pub negative: bool,
    pub base: String,
    pub value: Option<String>,
    pub arbitrary: bool,
    pub arbitrary_property: bool,
    pub is_known: bool,
}

impl ParsedClass {
    pub fn variant_level(&self) -> VariantLevel {
        if self.variants.is_empty() {
            return VariantLevel::None;
        }
        let has_screen = self.variants.iter().any(|v| matches!(v, Variant::Screen(_) | Variant::ArbitraryMedia(_)));
        if has_screen {
            VariantLevel::Screen
        } else {
            VariantLevel::State
        }
    }

    pub fn screen_name(&self) -> Option<&str> {
        self.variants.iter().find_map(|v| match v {
            Variant::Screen(name) => Some(name.as_str()),
            _ => None,
        })
    }
}

impl fmt::Display for ParsedClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for variant in &self.variants {
            match variant {
                Variant::Screen(name) | Variant::State(name) => {
                    write!(f, "{name}{SEPARATOR}")?;
                }
                Variant::ArbitrarySelector(sel) => {
                    write!(f, "{sel}{SEPARATOR}")?;
                }
                Variant::ArbitraryMedia(media) => {
                    write!(f, "{media}{SEPARATOR}")?;
                }
                Variant::Direction => {
                    write!(f, "rtl{SEPARATOR}")?;
                }
            }
        }

        if self.importance {
            write!(f, "!")?;
        }

        if self.arbitrary_property {
            write!(f, "[{}]", self.value.as_deref().unwrap_or(""))?;
        } else {
            if self.negative {
                write!(f, "-")?;
            }
            write!(f, "{}", self.base)?;
            if let Some(ref value) = self.value {
                write!(f, "-{}", value)?;
            }
        }

        Ok(())
    }
}

pub fn parse_class(input: &str) -> Option<ParsedClass> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut importance = false;
    let mut s = trimmed;

    if let Some(rest) = s.strip_prefix('!') {
        importance = true;
        s = rest;
    }

    let mut variants = Vec::new();
    loop {
        if let Some(rest) = s.strip_prefix(&format!("rtl{SEPARATOR}")) {
            variants.push(Variant::Direction);
            s = rest;
            continue;
        }
        if let Some((variant, rest)) = parse_variant(s) {
            variants.push(variant);
            s = rest;
            continue;
        }
        break;
    }

    let (base, value, arbitrary, arbitrary_property, negative) = parse_utility(s);

    let is_known = if arbitrary_property {
        true
    } else {
        !base.is_empty()
            && crate::sort_order::classify_utility(&base)
                != crate::sort_order::UtilityGroup::Unknown
    };

    Some(ParsedClass {
        variants,
        importance,
        negative,
        base,
        value,
        arbitrary,
        arbitrary_property,
        is_known,
    })
}

pub fn parse_style_variant(s: &str) -> Option<(&str, bool)> {
    if s.is_empty() {
        return None;
    }

    if let Some(rest) = s.strip_prefix(SEPARATOR) {
        return Some((rest, true));
    }

    if s.starts_with('[')
        && let Some(bracket_end) = s.find(']') {
            let after_bracket = &s[bracket_end + 1..];
            if let Some(rest) = after_bracket.strip_prefix(SEPARATOR) {
                return Some((rest, true));
            }
        }

    if !s.starts_with('@') {
        let sep_pos = s.find(SEPARATOR)?;
        let rest = &s[sep_pos + SEPARATOR.len()..];
        return Some((rest, true));
    }

    None
}

fn parse_variant(input: &str) -> Option<(Variant, &str)> {
    if input.is_empty() {
        return None;
    }

    if input.starts_with("[&")
        || input.starts_with("[data-")
        || input.starts_with("[aria-")
        || input.starts_with("[open]")
        || input.starts_with("[checked]")
        || input.starts_with("[disabled]")
    {
        let close_idx = input.find(']')?;
        let selector = &input[1..close_idx];
        let rest = &input[close_idx + 1..];
        let rest = rest.strip_prefix(SEPARATOR)?;
        return Some((Variant::ArbitrarySelector(selector.to_string()), rest));
    }

    if input.starts_with("[@media(") || input.starts_with("[@container(") || input.starts_with("[@supports(") {
        let close_idx = input.find(']')?;
        let media = &input[1..close_idx];
        let rest = &input[close_idx + 1..];
        let rest = rest.strip_prefix(SEPARATOR)?;
        return Some((Variant::ArbitraryMedia(media.to_string()), rest));
    }

    if input.starts_with('[') {
        let close_idx = input.find(']')?;
        let selector = &input[1..close_idx];
        let rest = &input[close_idx + 1..];
        let rest = rest.strip_prefix(SEPARATOR)?;
        return Some((Variant::ArbitrarySelector(selector.to_string()), rest));
    }

    let sep_pos = input.find(SEPARATOR)?;
    let name = &input[..sep_pos];
    if name.is_empty() {
        return None;
    }

    if name.ends_with('*') || name == "group" || name == "peer" {
        let rest = &input[sep_pos + SEPARATOR.len()..];
        if let Some((_inner, remainder)) = parse_variant(rest) {
            return Some((Variant::State(format!("{}-", name)), remainder));
        }
        let colon2 = rest.find(SEPARATOR);
        if let Some(pos2) = colon2 {
            let inner_name = &rest[..pos2];
            let final_rest = &rest[pos2 + SEPARATOR.len()..];
            return Some((Variant::State(format!("{}-{}", name, inner_name)), final_rest));
        }
        return Some((Variant::State(name.to_string()), rest));
    }

    if name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        let known_screens = ["sm", "md", "lg", "xl", "2xl", "3xl", "print"];
        let rest = &input[sep_pos + SEPARATOR.len()..];
        if known_screens.contains(&name) {
            return Some((Variant::Screen(name.to_string()), rest));
        }
        return Some((Variant::State(name.to_string()), rest));
    }

    None
}

fn parse_utility(input: &str) -> (String, Option<String>, bool, bool, bool) {
    if input.is_empty() {
        return (String::new(), None, false, false, false);
    }

    if input.starts_with('[')
        && let Some(close_idx) = input.find(']') {
            let inner = &input[1..close_idx];
            return (String::new(), Some(inner.to_string()), true, true, false);
        }

    let s = input;

    let negative = s.starts_with('-');
    let s = if negative { &s[1..] } else { s };

    let base_end = s.find(['-', '[']).unwrap_or(s.len());
    let base = s[..base_end].to_string();

    if crate::sort_order::classify_utility(&base) == crate::sort_order::UtilityGroup::Unknown
        && !base.is_empty()
    {
        let full = if negative {
            format!("-{}", input)
        } else {
            input.to_string()
        };
        return (full, None, false, false, false);
    }

    let _rest = &s[base_end..];

    if _rest.is_empty() {
        return (base, None, false, false, negative);
    }

    if _rest.starts_with('[') {
        let value = _rest.to_string();
        return (base, Some(value), true, false, negative);
    }

    let value = _rest[1..].to_string();
    (base, Some(value), false, false, negative)
}
