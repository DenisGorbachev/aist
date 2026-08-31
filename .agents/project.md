# aist

## Cargo.toml

- Must contain workspace packages:
  - [aist-rust-analyzer](#aist-rust-analyzer)
- Must contain dependencies:
  - `clap`

## aist-rust-analyzer

- Must contain items:
  - [ListTypesAistCommand](#struct-listtypesaistcommand)
- Must have dependencies:
  - Any rust-analyzer crates

### struct AistCommand

- Must have fields:
  - `project_dir: PathBuf` (short = 'p')
- Must have methods:
  - `run`
    - Should create the vars that would be passed to the `run` methods of subcommands

### struct ListTypesAistCommand

- Must have methods:
  - `run`
    - Must list the types in the project
