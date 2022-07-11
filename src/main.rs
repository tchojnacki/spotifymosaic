mod api;
mod args;
mod auth;
mod cli;
mod images;

use args::parse_args;
use cli::run;

#[tokio::main]
async fn main() {
    let args = parse_args();

    if let Err(msg) = run(args).await {
        eprintln!("{msg}");
    }
}
