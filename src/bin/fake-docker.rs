//! impl a simple client like `docker` with crate `clap`
//! ```bash
//! docker log
//! docker ps
//! ```

use clap::Parser;

/// Example show how to use this demo
/// Subcommand:
/// - log
/// ```bash
/// ./fake-docker log -f --since "1h" -c 1fab3
/// ```
/// - ps
/// ```bash
/// ./fake-docker ps 1fab3
/// ```
#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommand enums
/// clap subcommand must be a enum
#[derive(clap::Subcommand, Debug)]
enum Commands {
    #[clap(name = "log", about = "show container logs")]
    Logs(CommandLog),
    #[clap(about = "show container status")]
    Ps(CommandPs),
}

/// log
#[derive(clap::Args, Debug)]
struct CommandLog {
    #[clap(short, long, help = "follow logs")]
    follow: bool,
    #[clap(long, help = r#"time eg:"15m","1h","2d""#)]
    since: Option<String>,
    #[clap(short, long, value_name = "CONTAINER ID")]
    container: String,
}

/// ps
#[derive(clap::Args, Debug)]
struct CommandPs {
    #[clap(value_name = "CONTAINER ID")]
    container: String,
}

fn main() {
    let args: Args = Args::parse();
    match args.command {
        Commands::Logs(ref v) => {
            println!("log:{:?}", v)
        }
        Commands::Ps(ref v) => {
            println!("ps:{:?}", v)
        }
    }
}
