use cargo_workspace_info::CargoWorkspaceInfo;
use clap::Clap;
use eyre::Result;

mod add_changeset;
mod cargo_workspace_info;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    /// Adds a changeset
    Add,
    // / Initialises a Changesets project
    // Init,
    /// Updates the versions of your crates based on your changesets
    Version,
}

fn version() -> Result<()> {
    unimplemented!()
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let cargo_workspace_info = CargoWorkspaceInfo::new(std::env::current_dir()?.as_path());

    match opts.subcmd.unwrap_or(SubCommand::Add) {
        SubCommand::Add => add_changeset::add_changeset(cargo_workspace_info),
        SubCommand::Version => version(),
    }
}
