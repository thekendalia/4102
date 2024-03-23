use serde::{Deserialize, Serialize};
use reqwest;
use reqwest::header;

#[derive(Debug, Deserialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize)]
struct Sys {
    #[serde(rename = "type")]
    sys_type: u32,
    id: u32,
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    rain: Option<Rain>,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i64,
    id: u64,
    name: String,
    cod: u32,
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    rain_1h: f64,
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    println!("Enter a city :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("City: {}", line);
    println!("Please wait...");

    let _user_name = String::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-RapidAPI-Host", "weather-api138.p.rapidapi.com".parse().unwrap());
    headers.insert("X-RapidAPI-Key", "b7b9396f11msh1ab0e0941d5e1e3p1582dejsn6bd8bdebbd1b".parse().unwrap());

    let client = reqwest::Client::builder()
        .build()?;
    
    let res = client.get(&format!("https://weather-api138.p.rapidapi.com/weather?city_name={}", line))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let weather_response: WeatherResponse = serde_json::from_str(&res)?;

    println!("Coordinates: {:?}", weather_response.coord);
    println!("Weather: {:?}", weather_response.weather);
    println!("Main: {:?}", weather_response.main);
    println!("Wind: {:?}", weather_response.wind);
    println!("Rain: {:?}", weather_response.rain);
    println!("Clouds: {:?}", weather_response.clouds);
    println!("Sys: {:?}", weather_response.sys);
    println!("Name: {}", weather_response.name);

    Ok(())
} 