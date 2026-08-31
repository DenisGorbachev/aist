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

- Must contain items:
  - [Command](#struct-command)
  - [ListTypesCommand](#struct-listtypescommand)
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

- Must contain items:
  - [WorkspaceInfo](#struct-workspaceinfo)

## struct WorkspaceInfo

- Must have fields:
  - `db: RootDatabase`
  - `vfs: Vfs`
  - `proc_macro_client: ProcMacroClient`
- Must have impls:
  - `impl TryFrom<&Path> for WorkspaceInfo`
    - Must call `load_workspace_at` with constant `cargo_config`, constant `load_config`, dummy `progress`

## aist-spec package

- Must contain items:
  - [SpecCommand](#struct-speccommand)
- Must contain functions:
  - `main`
    - `let command = SpecCommand::parse()`
    - `let result = command.run().await`

## struct SpecCommand

- Must have fields:
  - `project_root: Option<PathBuf>` (short = 'p')
  - `output_format: Format` (`default_value_t = Format::Yaml`)
- Must have methods:
  - `run`
    - `let project_root = unwrap_or_current_dir(project_root)`
    - `let report = SpecReport::new()`
    - Must output `report`

## struct SpecReport

- Must have fields:
  - `aist: Result<AistPackage, AistPackageNewError>`
- Must have methods:
  - `new(ws: &WorkspaceInfo)`

Notes:

- The purpose of this struct is to report the current state of the workspace being passed as `project_root`

## struct AistPackage

- Must have fields:
  - `command: Result<StructCommand, StructCommandNewError>`
- Must have methods:
  - `new(ws: &WorkspaceInfo)`
    - `let command = StructCommand::new(ws)`

## struct StructCommand

- Must not have fields
- Must have methods:
  - `new(ws: &WorkspaceInfo)`
    - Must call `find_type("Command", &ws.db)`

## fn find_type

- Must have inputs:
  - `name: &str`
  - `db: &RootDatabase`
- Must have output: (TODO: determine the output type; should be an identifier of the type found by `name`)
- Must find the type by `name`
- Must check that the type is unique
