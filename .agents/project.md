# aist

## Cargo.toml

- Must contain workspace packages:
  - [aist-rust-analyzer](#aist-rust-analyzer)
- Must contain dependencies:
  - `clap`
  - `save-load`

## aist-rust-analyzer

- Must contain items:
  - [ListTypesAistCommand](#struct-listtypesaistcommand)
- Must have dependencies:
  - Any rust-analyzer crates
  - `save-load`

### struct AistCommand

- Must have fields:
  - `project_dir: PathBuf` (short = 'p')
  - `output_format: Format` (`default_value_t = Format::Yaml`)
- Must have methods:
  - `run`
    - Should create the vars that would be passed to the `run` methods of subcommands

### struct ListTypesAistCommand

- Must have methods:
  - `run`
    - Must list the types in the project
    - Must include types produced by declarative and procedural macro expansion when rust-analyzer can expand the macro
    - Must serialize the type listing using `output_format`
