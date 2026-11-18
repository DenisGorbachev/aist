use ra_ap_hir::{Adt, GenericDef, HasCrate, Impl, Trait, attach_db};
use ra_ap_ide_db::RootDatabase;

pub fn has_trait_impl(adt: Adt, required: Trait, db: &RootDatabase) -> bool {
    attach_db(db, || {
        let ty = adt.ty(db);
        let generic_adt = !GenericDef::from(adt).params(db).is_empty();
        let generic_trait = required.type_or_const_param_count(db, false) != 0 || !GenericDef::from(required).lifetime_params(db).is_empty();
        let proven = (!generic_trait && ty.impls_trait(db, required, &[])) || ((generic_trait || generic_adt) && ty.has_any_impl(db, required, &[]));
        if proven || !generic_adt {
            return proven;
        }
        let krate = adt.krate(db);
        Impl::all_for_type(db, ty)
            .into_iter()
            .any(|item| item.trait_(db) == Some(required) && item.module(db).krate(db) == krate && !item.is_negative(db) && item.self_ty(db).as_adt() == Some(adt))
    })
}
