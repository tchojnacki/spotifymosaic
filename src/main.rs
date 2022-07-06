use clap::{clap_derive::ArgEnum, value_parser, AppSettings, ArgGroup, Parser};
use rspotify::{ClientCredsSpotify, Credentials};
use spotifymosaic::generate_mosaic;
use std::path::PathBuf;

#[non_exhaustive]
#[derive(Debug, Clone, ArgEnum)]
pub enum TileArrangementArg {
    First,
    Random,
}

#[derive(Parser, Debug)]
#[clap(
    about, long_about = None,
    setting(AppSettings::DeriveDisplayOrder),
    group(ArgGroup::new("authorization").required(true))
)]
/// Generate a mosaic for a given Spotify playlist using album artworks
struct CliArgs {
    #[clap(value_parser)]
    /// Spotify playlist URI
    pub playlist_uri: String,

    #[clap(
        long = "creds",
        value_parser,
        group = "authorization",
        value_name = "CLIENT_ID:CLIENT_SECRET"
    )]
    /// Spotify client's ID and secret delimited by a colon
    pub credentials: String,

    #[clap(short, long = "tiles", value_parser = value_parser!(u8).range(1..=100), default_value_t = 2)]
    /// Mosaic's side length
    pub tile_count: u8,

    #[clap(short, long = "out", value_parser, default_value = "mosaic.png")]
    /// Output image file path
    pub output_path: PathBuf,

    #[clap(
        short,
        long = "arrange",
        arg_enum,
        value_parser,
        default_value_t = TileArrangementArg::First
    )]
    /// Order of mosaic's squares
    pub arrangement: TileArrangementArg,

    #[clap(short, long = "res", value_parser = value_parser!(u16).range(16..=4096), default_value_t = 640)]
    /// Output image's resolution
    pub resolution: u16,

    #[clap(short, long, value_parser)]
    /// Print all logs
    pub verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    println!("{:?}", args);

    if let Err(msg) = run(args).await {
        eprintln!("{}", msg);
    }
}

async fn run(args: CliArgs) -> Result<(), &'static str> {
    let (id, secret) = args
        .credentials
        .split_once(':')
        .ok_or("Invalid credentials format")?;

    let creds = Credentials::new(id, secret);
    let mut client = ClientCredsSpotify::new(creds);

    client
        .request_token()
        .await
        .or(Err("Authentication failed!"))?;

    generate_mosaic(&client, &args.playlist_uri).await.unwrap();

    Ok(())
}
