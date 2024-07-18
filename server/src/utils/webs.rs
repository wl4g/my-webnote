use axum::{
    body::Body,
    extract::Request,
    http::{ header, HeaderMap, HeaderValue, Response },
    response::{ IntoResponse, Redirect },
};
use hyper::StatusCode;
use tower_cookies::{ cookie::{ time::Duration, CookieBuilder, SameSite }, Cookie };

pub const APPLICATION_JSON_HEADER_VALUE: HeaderValue = HeaderValue::from_static("application/json");

pub fn create_cookie_headers(key: &str, value: &str) -> header::HeaderMap {
    let cookie = CookieBuilder::new(key, value)
        .path("/")
        .max_age(Duration::seconds(60))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();
    let header_value = cookie.to_string().parse::<HeaderValue>().expect("Failed to parse cookie");
    let mut headers = header::HeaderMap::new();
    headers.append(header::SET_COOKIE, header_value); // Will cover!
    headers
}

pub fn add_cookies(response: &mut Response<Body>, cookies: Vec<Cookie>) {
    cookies.iter().for_each(|c| {
        response
            .headers_mut()
            .append(header::SET_COOKIE, HeaderValue::from_str(&c.to_string()).unwrap());
    });
}

pub fn get_cookie_from_req(key: &str, req: &Request<Body>) -> Option<String> {
    get_cookie_from_headers(key, req.headers())
}

pub fn get_cookie_from_headers(key: &str, headers: &HeaderMap) -> Option<String> {
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

pub fn is_browser(headers: &HeaderMap) -> bool {
    let user_agent = headers
        .get("User-Agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    user_agent.contains("Mozilla")
}

pub fn response_redirect_or_json(
    status: StatusCode,
    headers: &HeaderMap,
    cookies: Option<(Cookie, Cookie)>,
    redirect_url: &str,
    message: &str,
    json: &str
) -> Response<Body> {
    let mut response;
    if is_browser(headers) {
        let mut _url;
        if status == StatusCode::OK {
            _url = redirect_url.to_owned();
        } else {
            _url = format!("{}#help-troubleshooting-is-{}", redirect_url, message).to_string();
        }
        // Refer to github authorization troubleshooting reason tips.
        // let url = match url::Url::parse(redirect_url).as_mut() {
        //     Ok(_url) => {
        //         _url.set_fragment(Some(_url.as_str()));
        //         _url.to_string()
        //     }
        //     Err(e) => {
        //         println!("url parse error:{}", e);
        //         "/".to_string() // TODO: redirect to error or default?
        //     }
        // };
        response = Redirect::to(_url.as_str()).into_response();
    } else {
        response = (status, json.to_string()).into_response();
        response.headers_mut().insert(header::CONTENT_TYPE, APPLICATION_JSON_HEADER_VALUE);
    }
    if let Some(pair) = cookies {
        add_cookies(&mut response, vec![pair.0, pair.1]);
    }
    response
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_get_cookie_from_headers() {
        let headers = &mut header::HeaderMap::new();
        let cookie = get_cookie_from_headers("test", headers);
        assert_eq!(cookie, None);
    }
    #[test]
    fn test_get_cookie_from_headers_with_cookie() {
        let headers = &mut header::HeaderMap::new();
        headers.insert("Cookie", "test=test".parse().unwrap());
        let cookie = get_cookie_from_headers("test", headers);
        assert_eq!(cookie, Some("test".to_string()));
    }
    #[test]
    fn test_get_cookie_from_headers_with_multiple_cookies() {
        let headers = &mut header::HeaderMap::new();
        headers.insert("Cookie", "test=test; test2=test2".parse().unwrap());
        let cookie = get_cookie_from_headers("test", headers);
        assert_eq!(cookie, Some("test".to_string()));
    }
}
