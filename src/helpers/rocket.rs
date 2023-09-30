use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{self, Responder, Response};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub message: String,
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiVersion {
    pub version: String,
    pub app_name: String,
    pub current_time: String,
    pub current_host: String,
    pub client_ip: String,
}

#[derive(Debug)]
pub struct RequestSocketAddr {
    pub socket_addr: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestSocketAddr {
    type Error = std::convert::Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(socket_addr) = req.headers().get_one("X-Forwarded-For") {
            return Outcome::Success(RequestSocketAddr {
                socket_addr: socket_addr.to_string(),
            });
        }
        if let Some(socket_addr) = req.remote() {
            return Outcome::Success(RequestSocketAddr {
                socket_addr: socket_addr.to_string(),
            });
        }
        Outcome::Success(RequestSocketAddr {
            socket_addr: "255.255.255.255".to_owned(),
        })
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let json = rocket::serde::json::Json(self);
        Response::build_from(json.respond_to(req)?)
            .status(rocket::http::Status::InternalServerError)
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
