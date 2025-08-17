use reqwest::Url;
use serde_json::Value;
const API_KEY: &str = "cf919ec7a29d5ab4c5c31899c19e7987";

pub async fn fetch_current_weather(city: &str) -> Value {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, API_KEY
    );
    let url = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            panic!("Invalid URL: {}", e);
        }
    };

    let res = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => {
            panic!("Request failed: {}", e);
        }
    };

    match res.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(e) => panic!("Failed to parse JSON: {}", e),
    }
}
