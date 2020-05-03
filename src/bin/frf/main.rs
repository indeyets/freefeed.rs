use std::env;
use std::process::exit;
use std::ops::Deref;

use clap::Clap;
use freefeed::api::api_client;

#[derive(Clap)]
struct Opts {
    /// Specify API origin. Will fall back to FRF_ORIGIN or "https://candy.freefeed.net/"
    #[clap(short, long)]
    origin: Option<String>,
    /// Specify your API Token. Will fall-back to FRF_TOKEN
    #[clap(short, long)]
    token: Option<String>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    Me(MeOpts)
}

#[derive(Clap)]
struct MeOpts {
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let origin = match opts.origin {
        Some(val) => val,
        None => match env::var("FRF_ORIGIN") {
            Ok(val) => val,
            Err(_e) => String::from("https://candy.freefeed.net"),
        }
    };

    let token: Option<String>  = match opts.token {
        Some(val) => Some(val),
        None => match env::var("FRF_TOKEN") {
            Ok(val) => Some(val),
            Err(_e) => {
                eprintln!("FRF_TOKEN env variable is not found\nConsider using https://direnv.net/ to set it\n\nWill work in anonymous mode");
                None
            }
        }
    };

    match opts.command {
        Command::Me(_) => {
            let client = api_client(origin.deref(), token.as_deref());
            match client.get_me().await {
                Ok(val) => println!("{}", val),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
    }
}
