use clap::Command;
use tracing::trace;

use crate::app::App;

const COMMAND_ROOT: &str = "netheril";

pub fn cmd() -> Command {
    Command::new(COMMAND_ROOT)
        .about(format!("{} - a city for your application", COMMAND_ROOT))
        .bin_name(COMMAND_ROOT)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(server_cmd())
        .subcommand(watch_cmd())
}

fn server_cmd() -> Command {
    Command::new("server").about("run the server")
}

#[derive(Debug, Clone)]
struct ServerCmdArgs {}

async fn execute_server(args: ServerCmdArgs) -> Result<(), Box<dyn std::error::Error>> {
    trace!("execute_server: {:?}", args);

    let app = App::new();
    app.run().await?;
    Ok(())
}

fn watch_cmd() -> Command {
    Command::new("watch").about("watch the server")
}

#[derive(Debug, Clone)]
struct WatchCmdArgs {}

async fn execute_watch(args: WatchCmdArgs) -> Result<(), Box<dyn std::error::Error>> {
    trace!("execute_watch: {:?}", args);

    Ok(())
}

pub async fn handle_cli() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cmd().get_matches();
    match matches.subcommand() {
        Some(("server", _)) => execute_server(ServerCmdArgs {}).await,
        Some(("watch", _)) => execute_watch(WatchCmdArgs {}).await,
        _ => unreachable!(),
    }
}
