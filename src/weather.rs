use serde::{Deserialize};
use reqwest;
use reqwest::header;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Main {
    pub temp: f64,
    feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Rain {
    #[serde(rename = "1h")]
    rain_1h: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Clouds {
    all: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Sys {
    #[serde(rename = "type")]
    sys_type: u32,
    id: u32,
    country: String,
    sunrise: u64,
    sunset: u64,

}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub sys: Sys,
    pub name: String,
}

pub async fn get_weather(city: &str) -> Result<WeatherResponse, Box<dyn std::error::Error + Send + Sync>> {
    let mut headers = header::HeaderMap::new();
    let api_key = env::var("API_KEY")?;
    headers.insert("X-RapidAPI-Host", "weather-api138.p.rapidapi.com".parse().unwrap());
    headers.insert("X-RapidAPI-Key", api_key.parse().unwrap());

    let client = reqwest::Client::builder()
        .build()?;
    
    let res = client.get(&format!("https://weather-api138.p.rapidapi.com/weather?city_name={}", city))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let weather_response: WeatherResponse = serde_json::from_str(&res)?;

    Ok(weather_response)
}