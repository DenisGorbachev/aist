use crate::normalize_fields;
use quote::ToTokens;
use syn::{Item, TraitItem};

pub fn item_shape(mut item: Item) -> String {
    use Item::*;
    match &mut item {
        Struct(item) => {
            item.attrs.clear();
            item.generics.params.pop_punct();
            normalize_fields(&mut item.fields);
        }
        Enum(item) => {
            item.attrs.clear();
            item.generics.params.pop_punct();
            item.variants.pop_punct();
            item.variants.iter_mut().for_each(|variant| {
                variant.attrs.clear();
                normalize_fields(&mut variant.fields);
            });
        }
        Type(item) => {
            item.attrs.clear();
            item.generics.params.pop_punct();
        }
        Trait(item) => {
            item.attrs.clear();
            item.items.iter_mut().for_each(|item| match item {
                TraitItem::Type(item) => item.attrs.clear(),
                TraitItem::Fn(item) => item.attrs.clear(),
                _ => (),
            });
        }
        Macro(item) => item.attrs.retain(|attr| !attr.path().is_ident("doc")),
        _ => (),
    }
    item.into_token_stream().to_string()
}
