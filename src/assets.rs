use rocket::response::content::{RawJavaScript, RawCss, RawHtml};

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
