use axum::{ body::Body, extract::Request, http::{ header, HeaderValue } };
use tower_cookies::cookie;

pub fn create_cookie_headers(key: &str, value: &str) -> header::HeaderMap {
  let mut headers = header::HeaderMap::new();
  add_cookie(key, value, &mut headers);
  headers
}

pub fn add_cookie(key: &str, value: &str, headers: &mut header::HeaderMap) {
  let header_value = format!("{}={}; Path=/; Version=1;", key, value)
    .parse::<HeaderValue>()
    .expect("Failed to parse cookie");
  headers.append(header::SET_COOKIE, header_value); // 会覆盖
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
  headers.get(header::COOKIE).and_then(|cookie_header| {
    cookie_header
      .to_str()
      .ok()
      .and_then(|cookie_str| get_cookie_from_str(cookie_str, key))
  })
}

pub fn get_cookie_from_str(cookie_str: &str, key: &str) -> Option<String> {
  cookie_str
    .split(';')
    .map(|pair| {
      let mut parts = pair.trim().splitn(2, '=');
      let name = parts.next().unwrap_or("").to_string();
      let value = parts.next().unwrap_or("").to_string();
      (name, value)
    })
    .find(|(name, _)| name == key)
    .map(|(_, value)| value)
}

mod tests {
  #[allow(unused)]
  use super::*;

  #[test]
  fn test_get_cookie_from_headers() {
    let headers = header::HeaderMap::new();
    let cookie = get_cookie_from_headers("test", headers);
    assert_eq!(cookie, None);
  }
  #[test]
  fn test_get_cookie_from_headers_with_cookie() {
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", "test=test".parse().unwrap());
    let cookie = get_cookie_from_headers("test", headers);
    assert_eq!(cookie, Some("test".to_string()));
  }
  #[test]
  fn test_get_cookie_from_headers_with_multiple_cookies() {
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", "test=test; test2=test2".parse().unwrap());
    let cookie = get_cookie_from_headers("test", headers);
    assert_eq!(cookie, Some("test".to_string()));
  }

  //   #[test]
  //   fn test_add_cookie() {
  //     let mut headers = header::HeaderMap::new();
  //     add_cookie("_ak", "abcd", &mut headers);
  //     add_cookie("_rk", "efgh", &mut headers);
  //     let result = cookie::Cookie
  //       ::parse(headers.get("Set-Cookie").unwrap().to_str().unwrap())
  //       .unwrap();
  //     println!("{:?}", result);
  //     assert_eq!(result.value(), "test");
  //   }
}
