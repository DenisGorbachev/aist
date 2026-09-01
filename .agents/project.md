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
    - Must find the unique local library crate named `aist`
    - Must create `aist_lib: Crate`
    - `let command = StructCommand::new(&aist_lib, &ws.db)`

### struct StructCommand

- Must not have fields
- Must have methods:
  - `new(lib: &Crate, db: &RootDatabase) -> Result<Self, StructCommandNewError>`
    - Must call `find_struct("Command", lib, db)`

### fn find_struct

- Must have inputs:
  - `name: &str`
  - `krate: &Crate`
  - `db: &RootDatabase`
- Must have output: `Result<Struct, FindStructError>`
- Must find struct declarations by `name` only within `krate`
- Must ignore imports, aliases, and non-struct types
- Must check that the struct is unique
