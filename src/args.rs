use clap::{clap_derive::ArgEnum, value_parser, AppSettings, ArgGroup, Parser};
use std::path::PathBuf;

#[non_exhaustive]
#[derive(Debug, Clone, ArgEnum)]
pub enum TileArrangement {
    First,
    Last,
    Random,
}

#[derive(Parser, Debug)]
#[clap(
    about, long_about = None,
    setting(AppSettings::DeriveDisplayOrder),
    group(ArgGroup::new("authorization").required(true))
)]
/// Generate a mosaic for a given Spotify playlist using album artworks
pub struct CliArgs {
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

    #[clap(short, long = "tiles", value_parser = value_parser!(u32).range(1..=128), default_value_t = 2)]
    /// Mosaic's side length
    pub tile_side_len: u32,

    #[clap(short, long = "out", value_parser, default_value = "mosaic.png")]
    /// Output image file path
    pub output_path: PathBuf,

    #[clap(
        short,
        long = "arrange",
        arg_enum,
        value_parser,
        default_value_t = TileArrangement::First
    )]
    /// Order of mosaic's squares
    pub arrangement: TileArrangement,

    #[clap(short, long = "res", value_parser = value_parser!(u32).range(16..=4096), default_value_t = 640)]
    /// Output image's resolution
    pub resolution: u32,

    #[clap(short, long, value_parser)]
    /// Print all logs
    pub verbose: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
