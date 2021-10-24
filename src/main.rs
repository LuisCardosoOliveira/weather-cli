use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("foo"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

fn main() {
    let api_token = std::env::var("API_TOKEN").expect("expected there to be an api token.");
    let args: String = std::env::args().skip(1).collect();

    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .headers(construct_headers())
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>() // this is called `turbofish`
        .expect("expected the body to be a JSON");

    dbg!(response);
}
