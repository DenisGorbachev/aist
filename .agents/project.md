# aist

## Cargo.toml

- Must contain dependencies:
  - `clap`
  - `serde`
  - `save-load`
- Must define a [aist](#aist-package) as a root package

## aist package

- Must contain items:
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
