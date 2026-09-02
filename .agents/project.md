# aist

## Cargo.toml

- Must contain dependencies:
  - `clap`
  - `serde`
  - `save-load`
- Must define a [aist](#aist-package) as a root package
- Must define workspace packages
  - [aist-core](#aist-core-package)
  - [aist-spec](#aist-spec-package)

## aist package

- Must have dependencies:
  - Any rust-analyzer crates
  - `save-load`

### struct Command

- Must have fields:
  - `project_root: PathBuf` (short = 'p')
  - `output_format: Format` (`default_value_t = Format::Yaml`)
- Must have methods:
  - `run`
    - Should create the vars that would be passed to the `run` methods of subcommands

### struct ListTypesCommand

- Must have methods:
  - `run`
    - Must list the types in the project
    - Must include types produced by declarative and procedural macro expansion when rust-analyzer can expand the macro
    - Must serialize the type listing using `output_format`

## aist-core package

### struct WorkspaceInfo

- Must have fields:
  - `db: RootDatabase`
  - `vfs: Vfs`
  - `proc_macro_client: ProcMacroClient`
- Must have impls:
  - `impl TryFrom<&Path> for WorkspaceInfo`
    - Must call `load_workspace_at` with constant `cargo_config`, constant `load_config`, dummy `progress`
- Must have methods:
  - `find_crate(name: &str, file_name: &str) -> Result<Crate, WorkspaceInfoFindCrateError>`
    - Must call `find_crate(name, file_name, &self.db, &self.vfs)`

### fn find_crate

- Must have inputs:
  - `name: &str`
  - `file_name: &str`
  - `db: &RootDatabase`
  - `vfs: &Vfs`
- Must have output: `Result<Crate, FindCrateError>`
- Must find the unique local crate with the requested crate display name and crate-root file name

### fn filter_adt

- Must have inputs:
  - `name: &str`
  - `krate: &Crate`
  - `db: &RootDatabase`
- Must have output: `impl Iterator<Item = Adt>`
- Must filter ADT declarations by `name` only within `krate`
- Must ignore imports, aliases, and non-ADT types

### fn get_adt

- Must have inputs:
  - `name: &str`
  - `krate: &Crate`
  - `db: &RootDatabase`
- Must have output: `Result<Adt, GetAdtError>`
- Must call `filter_adt(name, krate, db)`
- Must check that the ADT is unique

### fn get_struct

- Must have inputs:
  - `name: &str`
  - `krate: &Crate`
  - `db: &RootDatabase`
- Must have output: `Result<Struct, GetStructError>`
- Must call `get_adt(name, krate, db)`
- Must return an error if the ADT is not a struct

## aist-spec package

- Must contain functions:
  - `main`
    - `let command = SpecCommand::parse()`

### struct SpecCommand

- Must have fields:
  - `project_root: Option<PathBuf>` (short = 'p')
  - `output_format: Format` (`default_value_t = Format::Yaml`)
- Must have methods:
  - `run`
    - `let project_root = handle!(unwrap_or_current_dir(project_root), UnwrapOrCurrentDirFailed)`
    - `let workspace_info = handle!(WorkspaceInfo::try_from(project_root.as_path()), TryFromFailed, project_root)`
    - `let report = SpecReport::new(&workspace_info)`
    - Must serialize and output `report` using `output_format`

### struct SpecReport

- Must have derives:
  - `Serialize`
- Must have fields:
  - `aist: Result<AistLibCrate, AistLibCrateNewError>`
- Must have methods:
  - `new(ws: &WorkspaceInfo) -> Self`
    - `let aist = AistLibCrate::new(ws)`

Notes:

- The purpose of this struct is to report the current state of the workspace being passed as `project_root`

### struct AistLibCrate

- Must have fields:
  - `command: Result<StructCommand, StructCommandNewError>`
- Must have methods:
  - `new(ws: &WorkspaceInfo) -> Result<Self, AistLibCrateNewError>`
    - `let aist_lib = handle!(ws.find_crate("aist", "lib.rs"), FindCrateFailed)`
    - Must create `aist_lib: Crate`
    - `let command = StructCommand::new(&aist_lib, &ws.db)`

### struct StructCommand

- Must not have fields
- Must have methods:
  - `new(lib: &Crate, db: &RootDatabase) -> Result<Self, StructCommandNewError>`
    - Must call `get_struct("Command", lib, db)`
