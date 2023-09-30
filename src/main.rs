#[macro_use]
extern crate rocket;

use helpers::rocket::{ApiError, ApiVersion, RequestSocketAddr};
use rocket::response::content::{RawJavaScript, RawCss, RawHtml};
use rocket::{form::Form, fs::TempFile, serde::json::Json, serde::Serialize};
use rocket::Config;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod helpers;

const INDEX_HTML : &str = include_str!("../pages/dist/index.html");
const MAIN_JS : &str = include_str!("../pages/dist/assets/main.js");
const MAIN_CSS : &str = include_str!("../pages/dist/assets/main.css");

#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

#[get("/assets/main.js")]
pub fn main_js() -> RawJavaScript<&'static str> {
    RawJavaScript(MAIN_JS)
}

#[get("/assets/main.css")]
pub fn main_css() -> RawCss<&'static str> {
    RawCss(MAIN_CSS)
}

#[get("/echo")]
pub fn echo(socket_addr: RequestSocketAddr) -> Json<ApiVersion> {
    let version = env!("CARGO_PKG_VERSION");
    let app_name = env!("CARGO_PKG_NAME");
    let current_time = chrono::Local::now().to_string();
    let current_host =
        hostname::get().map_or("unknown".to_string(), |h| h.to_string_lossy().to_string());
    let client_ip = socket_addr.socket_addr.to_string();
    Json(ApiVersion {
        version: version.to_string(),
        app_name: app_name.to_string(),
        current_time: current_time.to_string(),
        current_host,
        client_ip: client_ip.to_string(),
    })
}

#[derive(FromForm)]
struct Upload<'f> {
    pub file: TempFile<'f>,
}

#[derive(Serialize, Debug)]
struct FileUploadedSucessfully {
    pub file_name: String,
}

/** Route to upload files */
#[post("/upload", format = "multipart/form-data", data = "<form>")]
async fn upload_file(
    mut form: Form<Upload<'_>>,
) -> Result<Json<FileUploadedSucessfully>, ApiError> {
    let file_name = format!("uploads/{}", form.file.name().unwrap());
    info!(
        "Uploading: destination = {}, len: {}",
        file_name,
        form.file.len()
    );
    let result = form.file.persist_to(file_name.clone()).await;
    match result {
        Ok(_) => Ok(Json(FileUploadedSucessfully {
            file_name: file_name.to_string(),
        })),
        Err(e) => {
            return Err(ApiError {
                message: "Error uploading file".to_string(),
                error: e.to_string(),
            });
        }
    }
}

/** Route to return a list of files */
#[get("/list")]
async fn list_files() -> Result<Json<Vec<String>>, ApiError> {
    let files = rocket::tokio::fs::read_dir("uploads").await;
    match files {
        Ok(mut files) => {
            let mut result: Vec<String> = vec![];
            while let Ok(file) = files.next_entry().await {
                match file {
                    None => break,
                    Some(file) => {
                        result.push(file.file_name().to_string_lossy().to_string());
                    }
                }
            }
            Ok(Json(result))
        }
        Err(e) => {
            return Err(ApiError {
                message: "Error listing files".to_string(),
                error: e.to_string(),
            });
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // setup logging system
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // grants uploads directory exists
    let _ = rocket::tokio::fs::create_dir_all("uploads").await;

    // starting application
    let _config = Config::figment()
        .merge(("limits.file", 1 * 1024 * 1024 * 1024))
        .merge(("address", "0.0.0.0"));
    let _rocket = rocket::build()
        .configure(_config)
        .attach(helpers::rocket::CORS)
        .mount("/v1", routes![echo])
        .mount("/files", routes![upload_file, list_files])
        .mount("/", routes![index, main_js, main_css])
        .launch()
        .await?;
    Ok(())
}
