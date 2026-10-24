use ra_ap_syntax::ast::{HasName, Item};

pub fn item_name(item: &Item) -> Option<String> {
    use Item::*;
    match item {
        Struct(item) => item.name().map(|name| name.to_string()),
        Enum(item) => item.name().map(|name| name.to_string()),
        Trait(item) => item.name().map(|name| name.to_string()),
        TypeAlias(item) => item.name().map(|name| name.to_string()),
        MacroRules(item) => item.name().map(|name| name.to_string()),
        MacroDef(item) => item.name().map(|name| name.to_string()),
        _ => None,
    }
}
