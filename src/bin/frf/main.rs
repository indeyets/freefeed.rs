use std::env;
use std::ops::Deref;
use std::process::exit;

use clap::{Parser, Subcommand, Args};

use freefeed::api::api_client;

mod format;
use format::format_post;

#[derive(Parser)]
struct Opts {
    /// Specify API origin. Will fall back to FRF_ORIGIN or "https://candy.freefeed.net"
    #[arg(
        short,
        long,
        env = "FRF_ORIGIN",
        default_value = "https://candy.freefeed.net"
    )]
    origin: String,
    /// Specify your API Token. Will fall-back to FRF_TOKEN
    #[arg(short, long)]
    token: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Me(MeOpts),
    GetPost(GetPostOps),
}

#[derive(Args)]
struct MeOpts {}

#[derive(Args)]
struct GetPostOps {
    #[clap(required = true)]
    uuid: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let origin = opts.origin;

    let token: Option<String> = match opts.token {
        Some(val) => Some(val),
        None => match env::var("FRF_TOKEN") {
            Ok(val) => Some(val),
            Err(_e) => {
                eprintln!("FRF_TOKEN env variable is not found\nConsider using https://direnv.net/ to set it\n\nWill work in anonymous mode");
                None
            }
        },
    };

    let client = api_client(origin.deref(), token.as_deref());

    match opts.command {
        Command::Me(_) => match client.get_me().await {
            Ok(val) => println!("{}", val),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        },
        Command::GetPost(opts) => {
            match client.get_a_post(opts.uuid.as_str()).await {
                Ok(val) => {
                    format_post(val);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
            exit(0);
        }
    }
}
