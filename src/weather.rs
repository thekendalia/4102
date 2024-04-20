use reqwest;
use reqwest::header;
use serde::Deserialize;
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
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Wind {
    pub speed: f64,
    deg: u32,
}

impl Wind {
    //public method to get wind speed in meters per second
    pub fn get_speed_meters_per_sec(&self) -> f64 {
        self.speed
    }

    //public method to get wind speed in miles per hour
    pub fn get_speed_mph(&self) -> f64 {
        self.speed * 2.23694 // Convert m/s to mph
    }
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
    pub all: u32,
}

#[allow(dead_code)]
impl Clouds {
    pub fn get_cloud_coverage(&self) -> u32 {
        self.all
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Sys {
    #[serde(rename = "type")]
    sys_type: u32,
    id: u32,
    country: String,
    pub sunrise: u64,
    pub sunset: u64,
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

pub async fn get_weather(
    city: &str,
) -> Result<WeatherResponse, Box<dyn std::error::Error + Send + Sync>> {
    let mut headers = header::HeaderMap::new();
    let api_key = env::var("API_KEY")?;
    headers.insert(
        "X-RapidAPI-Host",
        "weather-api138.p.rapidapi.com".parse().unwrap(),
    );
    headers.insert("X-RapidAPI-Key", api_key.parse().unwrap());

    let client = reqwest::Client::builder().build()?;

    let res = client
        .get(&format!(
            "https://weather-api138.p.rapidapi.com/weather?city_name={}",
            city
        ))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let weather_response: WeatherResponse = serde_json::from_str(&res)?;

    Ok(weather_response)
}
