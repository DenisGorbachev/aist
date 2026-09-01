use aist_spec::SpecCommand;
use clap::Parser;
use errgonomic::exit_result;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let command = SpecCommand::parse();
    exit_result(command.run().await)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    SpecCommand::command().debug_assert();
}
