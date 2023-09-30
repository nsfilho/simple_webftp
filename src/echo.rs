use super::helpers::rocket::{ApiVersion, RequestSocketAddr};
use rocket::serde::json::Json;


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
