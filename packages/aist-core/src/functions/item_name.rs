use syn::Item;

pub fn item_name(item: &Item) -> Option<String> {
    use Item::*;
    match item {
        Struct(item) => Some(item.ident.to_string()),
        Enum(item) => Some(item.ident.to_string()),
        Trait(item) => Some(item.ident.to_string()),
        Type(item) => Some(item.ident.to_string()),
        Macro(item) => item.ident.as_ref().map(ToString::to_string),
        _ => None,
    }
}
