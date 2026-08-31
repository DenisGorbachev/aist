macro_rules! impl_list_types_aist_command {
    ($command:ty, $error:ident) => {
        impl $command {
            pub async fn run(self, project_dir: std::path::PathBuf) -> Result<std::process::ExitCode, $error> {
                use $error::*;
                let Self = self;
                let exit_code = errgonomic::handle!($crate::run_list_types::<$command>(&project_dir), RunListTypesFailed);
                Ok(exit_code)
            }
        }

        #[derive(thiserror::Error, Debug)]
        pub enum $error {
            #[error("failed to list types")]
            RunListTypesFailed { source: $crate::RunListTypesError },
        }
    };
}

macro_rules! impl_symbol_rust_type_definition_collector {
    ($command:ty) => {
        impl $crate::RustTypeDefinitionCollector for $command {
            fn rust_type_definitions(db: &ra_ap_ide_db::RootDatabase) -> impl Iterator<Item = $crate::RustTypeDefinition> + '_ {
                $crate::rust_type_definitions_from_symbols::<Self>(db)
            }
        }
    };
}

macro_rules! rust_type_kind_from_module_def {
    ($module_def:expr) => {{
        use ra_ap_hir::Adt::{Enum as EnumAdt, Struct as StructAdt, Union as UnionAdt};
        use ra_ap_hir::ModuleDef;
        use $crate::RustTypeKind::*;
        match $module_def {
            ModuleDef::Adt(EnumAdt(_)) => Some(Enum),
            ModuleDef::Adt(StructAdt(_)) => Some(Struct),
            ModuleDef::Adt(UnionAdt(_)) => Some(Union),
            ModuleDef::TypeAlias(_) => Some(TypeAlias),
            _ => None,
        }
    }};
}

pub(crate) use impl_list_types_aist_command;
pub(crate) use impl_symbol_rust_type_definition_collector;
pub(crate) use rust_type_kind_from_module_def;
