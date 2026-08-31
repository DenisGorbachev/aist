use crate::RustTypeDefinition;
use ra_ap_ide_db::RootDatabase;

pub trait RustTypeDefinitionCollector {
    fn rust_type_definitions(db: &RootDatabase) -> impl Iterator<Item = RustTypeDefinition> + '_;
}
