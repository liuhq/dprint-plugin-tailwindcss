use std::cmp::Ordering;

use crate::css_config::CssConfig;
use crate::parsed_class::{parse_class, ParsedClass, SideOrder};
use crate::sort_order;

pub fn sort_class_string(input: &str, config: &CssConfig) -> Option<String> {
    let mut classes: Vec<ParsedClass> = input
        .split_whitespace()
        .filter_map(parse_class)
        .collect();

    if classes.len() <= 1 {
        return None;
    }

    for class in &mut classes {
        if !class.is_known
            && !class.base.is_empty()
            && config.custom_utilities.contains(&class.base)
        {
            class.is_known = true;
        }
    }

    let original: Vec<String> = classes.iter().map(|c| format!("{}", c)).collect();

    classes.sort_by(|a, b| cmp(a, b, config));

    let sorted: Vec<String> = classes.iter().map(|c| format!("{}", c)).collect();

    if original == sorted {
        return None;
    }

    Some(sorted.join(" "))
}

fn cmp(a: &ParsedClass, b: &ParsedClass, config: &CssConfig) -> Ordering {
    a.is_known.cmp(&b.is_known)
        .then_with(|| a.variant_level().cmp(&b.variant_level()))
        .then_with(|| {
            if a.variant_level() == crate::parsed_class::VariantLevel::Screen
                && b.variant_level() == crate::parsed_class::VariantLevel::Screen
            {
                let a_order = a
                    .screen_name()
                    .map(|n| config.breakpoint_order(n))
                    .unwrap_or(usize::MAX);
                let b_order = b
                    .screen_name()
                    .map(|n| config.breakpoint_order(n))
                    .unwrap_or(usize::MAX);
                a_order.cmp(&b_order)
            } else {
                Ordering::Equal
            }
        })
        .then_with(|| {
            let a_group = utility_group_for(a, config);
            let b_group = utility_group_for(b, config);
            a_group.cmp(&b_group)
        })
        .then_with(|| {
            if sort_order::is_same_family(&a.base, &b.base) {
                let a_side = classify_override_for(&a.base);
                let b_side = classify_override_for(&b.base);
                a_side.cmp(&b_side)
            } else {
                Ordering::Equal
            }
        })
}

fn utility_group_for(class: &ParsedClass, config: &CssConfig) -> sort_order::UtilityGroup {
    if class.arbitrary_property {
        return sort_order::UtilityGroup::ArbitraryProperty;
    }
    if class.base.is_empty() {
        return sort_order::UtilityGroup::Unknown;
    }
    if config.custom_utilities.contains(&class.base) {
        return sort_order::UtilityGroup::Custom;
    }
    sort_order::classify_utility(&class.base)
}

fn classify_override_for(base: &str) -> SideOrder {
    sort_order::classify_override(base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css_config::CssConfig;

    #[test]
    fn test_sort_basic() {
        let config = CssConfig::default();
        let result = sort_class_string("p-4 flex bg-red-500 text-white", &config);
        assert_eq!(result, Some("flex p-4 text-white bg-red-500".to_string()));
    }

    #[test]
    fn test_sort_override() {
        let config = CssConfig::default();
        let result = sort_class_string("pt-2 p-4", &config);
        assert_eq!(result, Some("p-4 pt-2".to_string()));
    }

    #[test]
    fn test_sort_no_change() {
        let config = CssConfig::default();
        let result = sort_class_string("flex items-center p-4", &config);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sort_unknown_front() {
        let config = CssConfig::default();
        let result = sort_class_string("p-3 shadow-xl select2-dropdown", &config);
        assert_eq!(result, Some("select2-dropdown p-3 shadow-xl".to_string()));
    }
}
