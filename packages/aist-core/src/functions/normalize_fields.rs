use syn::Fields;

pub fn normalize_fields(fields: &mut Fields) {
    fields.iter_mut().for_each(|field| field.attrs.clear());
    match fields {
        Fields::Named(fields) => {
            fields.named.pop_punct();
        }
        Fields::Unnamed(fields) => {
            fields.unnamed.pop_punct();
        }
        Fields::Unit => (),
    }
}
