use clap::Command;

const COMMAND_ROOT: &str = "netheril";

pub fn cmd() -> Command {
    Command::new(COMMAND_ROOT)
        .about("netheril - a city for your application")
        .styles(CLAP_STYLING)
        .bin_name(COMMAND_ROOT)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(server())
        .subcommand(watch())
}

fn server() -> Command {
    Command::new("server").about("run the server")
}

struct ServerCmdArgs {}

fn execute_server(_args: ServerCmdArgs) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn watch() -> Command {
    Command::new("watch").about("watch the server")
}

struct RunCmdArgs {}

fn execute_run(_args: RunCmdArgs) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cmd().get_matches();
    match matches.subcommand() {
        Some(("server", _)) => execute_server(ServerCmdArgs {}),
        Some(("watch", _)) => execute_run(RunCmdArgs {}),
        _ => unreachable!(),
    }
}

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);
