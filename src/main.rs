use aist::AistCommand;
use clap::Parser;
use errgonomic::exit_result;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = AistCommand::parse();
    let result = args.run().await;
    exit_result(result)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    AistCommand::command().debug_assert();
}
