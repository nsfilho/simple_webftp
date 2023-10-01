#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use clap::Parser;
use rocket::{fs::FileServer, Config};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod assets;
pub mod echo;
pub mod files;
pub mod helpers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "{about-section}Authors: {author-with-newline}Version: {version}\n\n{usage-heading} {usage}\n\n{all-args} {tab}"
)]
pub struct Args {
    #[clap(short, long, default_value = "0.0.0.0", help = "Address to listen on")]
    pub address: String,

    #[clap(short, long, default_value = "8000", help = "Port to listen on")]
    pub port: u16,

    #[clap(
        short,
        long,
        default_value = "1024000000",
        help = "Max file size in bytes, default: 1G"
    )]
    pub max_file_size: u64,

    #[clap(
        short,
        long,
        default_value = "uploads",
        help = "Folder to store uploaded files"
    )]
    pub folder: PathBuf,
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // setup logging system
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // starting application
    let _definitions = Args::parse();
    let upload_folder = _definitions.folder.clone();
    let _ = rocket::tokio::fs::create_dir_all(upload_folder.clone()).await;
    let _config = Config::figment()
        .merge(("limits.file", _definitions.max_file_size))
        .merge(("limits.data-form", _definitions.max_file_size))
        .merge(("limits.form", _definitions.max_file_size))
        .merge(("address", _definitions.address.clone()))
        .merge(("port", _definitions.port));
    let _rocket = rocket::build()
        .configure(_config)
        .manage(_definitions)
        .attach(helpers::rocket::CORS)
        .mount("/v1", routes![echo::echo])
        .mount("/files", routes![files::file_upload, files::file_list])
        .mount("/files/download", FileServer::from(upload_folder))
        .mount(
            "/",
            routes![assets::index, assets::main_js, assets::main_css],
        )
        .launch()
        .await?;
    Ok(())
}
