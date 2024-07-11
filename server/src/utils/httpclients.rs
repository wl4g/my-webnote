use std::time::Duration;

pub fn build_default() -> reqwest::Client {
  reqwest::ClientBuilder
    ::new()
    .connect_timeout(Duration::new(3, 0))
    .read_timeout(Duration::new(6, 0))
    .timeout(Duration::new(6, 0))
    //.proxy(Proxy::http("http://127.0.0.1:1080").unwrap())
    .connection_verbose(false)
    .build()
    .unwrap()
}
