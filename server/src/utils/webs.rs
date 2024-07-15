use axum::{ body::Body, extract::Request, http::{ header, HeaderValue } };
use tower_cookies::cookie;

pub fn create_cookie_headers(key: &str, value: &String) -> header::HeaderMap {
  let mut headers = header::HeaderMap::new();
  let header_value = format!("{}={}; Path=/", key, value)
    .parse::<HeaderValue>()
    .expect("Failed to parse cookie");
  headers.insert(header::SET_COOKIE, header_value);
  headers
}

pub fn get_cookie_from_req(key: &str, req: Request<Body>) -> Option<String> {
  req
    .headers()
    .get(header::COOKIE)
    .and_then(|value| {
      value
        .to_str()
        .ok()
        .and_then(|cookie_str| cookie::Cookie::parse(cookie_str).ok())
        .and_then(|cookie| (
          if cookie.name() == key {
            Some(cookie.value().to_string())
          } else {
            None
          }
        ))
    })
}

pub fn get_cookie_from_headers(key: &str, headers: header::HeaderMap) -> Option<String> {
  headers.get(header::COOKIE).and_then(|value| {
    value
      .to_str()
      .ok()
      .and_then(|cookie_str| cookie::Cookie::parse(cookie_str).ok())
      .and_then(|cookie| if cookie.name() == key { Some(cookie.value().to_string()) } else { None })
  })
}
