use aderyn::{
    aderyn_is_currently_running_newest_version, lsp::spin_up_language_server,
    print_all_detectors_view, print_detail_view,
};
use aderyn_driver::driver::{self, Args};

use clap::{ArgGroup, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("stdout_dependent").requires("stdout")))]
pub struct CommandLineArgs {
    /// Print every detector available
    #[clap(subcommand, name = "registry")]
    registry: Option<RegistryCommand>,

    /// Foundry or Hardhat project root directory (or path to single solidity file)
    #[arg(default_value = ".")]
    root: String,

    /// Path to the source contracts. If not provided, the ROOT directory will be used.
    ///
    /// For example, in a foundry repo:
    ///
    ///     --src=src/
    ///
    /// In a hardhat repo:
    ///
    ///    --src=contracts/
    #[clap(short, long, use_value_delimiter = true)]
    src: Option<Vec<String>>,

    /// List of path strings to include, delimited by comma (no spaces).
    /// Any solidity file path not containing these strings will be ignored
    #[clap(short = 'i', long, use_value_delimiter = true)]
    path_includes: Option<Vec<String>>,

    /// List of path strings to exclude, delimited by comma (no spaces).
    /// Any solidity file path containing these strings will be ignored
    #[clap(short = 'x', long, use_value_delimiter = true)]
    path_excludes: Option<Vec<String>>,

    /// Desired file path for the final report (will overwrite existing one)
    #[arg(short, long, default_value = "report.md")]
    output: String,

    /// [BETA] Start Aderyn's LSP server on stdout
    #[arg(short, long, group = "stdout_dependent")]
    lsp: bool,

    /// Do not include code snippets in the report (reduces report size in large repos)
    #[arg(long)]
    no_snippets: bool,

    /// Only use the high detectors
    #[arg(long)]
    highs_only: bool,

    /// Print the output to stdout instead of a file
    #[arg(long, name = "stdout")]
    stdout: bool,

    /// Skip cloc analysis (line numbers, etc.)
    #[arg(long)]
    skip_cloc: bool,

    /// Skip checking for new versions of Aderyn
    #[arg(long)]
    skip_update_check: bool,

    /// Run in Auditor mode, which only outputs manual audit helpers
    #[arg(long)]
    auditor_mode: bool,
}

#[derive(Debug, Subcommand)]
enum RegistryCommand {
    /// Browse detector registry
    Registry {
        /// all    - View all available detectors
        ///
        /// <name> - Detail view of a single detector
        #[arg(default_value = "all")]
        detector: String,
    },
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    if let Some(reg) = cmd_args.registry {
        match reg {
            RegistryCommand::Registry { detector } => {
                if detector == "all" {
                    print_all_detectors_view();
                } else {
                    print_detail_view(&detector);
                }
            }
        }
        return;
    }

    let mut args: Args = Args {
        root: cmd_args.root,
        output: cmd_args.output,
        src: cmd_args.src,
        path_includes: cmd_args.path_includes,
        path_excludes: cmd_args.path_excludes,
        no_snippets: cmd_args.no_snippets,
        skip_cloc: cmd_args.skip_cloc,
        skip_update_check: cmd_args.skip_update_check,
        stdout: cmd_args.stdout,
        auditor_mode: cmd_args.auditor_mode,
        highs_only: cmd_args.highs_only,
        lsp: cmd_args.lsp,
    };

    // Run watcher is watch mode is engaged
    if cmd_args.lsp {
        args.skip_cloc = true;
        args.skip_update_check = true;
        spin_up_language_server(args);
    } else {
        driver::drive(args.clone());
    }

    // Check for updates
    if !cmd_args.skip_update_check {
        if let Ok(yes) = aderyn_is_currently_running_newest_version() {
            if !yes {
                println!();
                println!("NEW VERSION OF ADERYN AVAILABLE! Please run `cyfrinup` to upgrade.");
            }
        }
    }
}
