use spotifymosaic::{parse_args, run};

#[tokio::main]
async fn main() {
    let args = parse_args();

    if let Err(msg) = run(args).await {
        eprintln!("{msg}");
    }
}
