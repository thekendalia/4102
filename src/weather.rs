use reqwest;
use reqwest::header;
use serde::Deserialize;
use std::env;
use rand::{Rng, thread_rng};

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
    pub country: String,
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
        .await?;
    
    if res.status().is_success() {
        let body = res.text().await?;
        let weather_response: WeatherResponse = serde_json::from_str(&body)?;
        Ok(weather_response)
    } else {
        Err(format!("Request failed with status code: {}", res.status()).into())
    }
}
pub fn get_random_city() -> (&'static str, &'static str, &'static str) {
    let cities = vec![
        ("New York", "USA", "🇺🇸"), ("Los Angeles", "USA", "🇺🇸"), ("Miami", "USA", "🇺🇸"), ("Honolulu", "USA", "🇺🇸"),
        ("Vancouver", "Canada", "🇨🇦"), ("Toronto", "Canada", "🇨🇦"), ("Mexico City", "Mexico", "🇲🇽"), ("Monterrey", "Mexico", "🇲🇽"),
        ("London", "UK", "🇬🇧"), ("Manchester", "UK", "🇬🇧"), ("Munich", "Germany", "🇩🇪"), ("Berlin", "Germany", "🇩🇪"),
        ("Madrid", "Spain", "🇪🇸"), ("Barcelona", "Spain", "🇪🇸"), ("Milan", "Italy", "🇮🇹"), ("Rome", "Italy", "🇮🇹"),
        ("Paris", "France", "🇫🇷"), ("Marseille", "France", "🇫🇷"), ("Glasgow", "UK", "🇬🇧"),
        ("Copenhagen", "Denmark", "🇩🇰"), ("Oslo", "Norway", "🇳🇴"), ("Stockholm", "Sweden", "🇸🇪"), ("Helsinki", "Finland", "🇫🇮"),
        ("Moscow", "Russia", "🇷🇺"), ("Kyiv", "Ukraine", "🇺🇦"), ("Warsaw", "Poland", "🇵🇱"), ("Prague", "Czechia", "🇨🇿"),
        ("Amsterdam", "Netherlands", "🇳🇱"), ("Brussels", "Belgium", "🇧🇪"), ("Bern", "Switzerland", "🇨🇭"), ("Vienna", "Austria", "🇦🇹"),
        ("Budapest", "Hungary", "🇭🇺"), ("Zagreb", "Croatia", "🇭🇷"), ("Bucharest", "Romania", "🇷🇴"), ("Athens", "Greece", "🇬🇷"),
        ("Lisbon", "Portugal", "🇵🇹"), ("Istanbul", "Turkey", "🇹🇷"), ("Casablanca", "Morocco", "🇲🇦"), ("Baku", "Azerbaijan", "🇦🇿"),
        ("Cairo", "Egypt", "🇪🇬"), ("Riyadh", "Saudi Arabia", "🇸🇦"), ("Doha", "Qatar", "🇶🇦"), ("Dubai", "UAE", "🇦🇪"),
        ("Muscat", "Oman", "🇴🇲"), ("Tehran", "Iran", "🇮🇷"), ("Lagos", "Nigeria", "🇳🇬"), ("Dakar", "Senegal", "🇸🇳"), ("Cape Town", "South Africa", "🇿🇦"),
        ("Mumbai", "India", "🇮🇳"), ("Dhaka", "Bangladesh", "🇧🇩"), ("Bangkok", "Thailand", "🇹🇭"), ("Kuala Lumpur", "Malaysia", "🇲🇾"),
        ("Tokyo", "Japan", "🇯🇵"), ("Seoul", "South Korea", "🇰🇷"), ("Beijing", "China", "🇨🇳"), ("Shanghai", "China", "🇨🇳"),
        ("Sydney", "Australia", "🇦🇺"), ("Perth", "Australia", "🇦🇺"), ("Melbourne", "Australia", "🇦🇺"), ("Auckland", "New Zealand", "🇳🇿"),
        ("Reykjavík", "Iceland", "🇮🇸"), ("Nairobi", "Kenya", "🇰🇪"), ("Harare", "Zimbabwe", "🇿🇼"), ("Luanda", "Angola", "🇦🇴"),
        ("Libreville", "Gabon", "🇬🇦"), ("Lusaka", "Zambia", "🇿🇲"), ("Freetown", "Sierra Leone", "🇸🇱"), ("Algiers", "Algeria", "🇩🇿"),
        ("Rio de Janeiro", "Brazil", "🇧🇷"), ("Buenos Aires", "Argentina", "🇦🇷"), ("Montevideo", "Uruguay", "🇺🇾"),
        ("Santiago", "Chile", "🇨🇱"), ("Lima", "Peru", "🇵🇪"), ("La Paz", "Bolivia", "🇧🇴"), ("Medellín", "Colombia", "🇨🇴"),
        ("Panama City", "Panama", "🇵🇦"), ("San Jose", "Costa Rica", "🇨🇷"), ("San Salvador", "El Salvador", "🇸🇻"),
        ("Havana", "Cuba", "🇨🇺"), ("Guatemala City", "Guatemala", "🇬🇹"), ("Tegucigalpa", "Honduras", "🇭🇳"), ("Managua", "Nicaragua", "🇳🇮"),
        ("Kingston", "Jamaica", "🇯🇲"), ("Santo Domingo", "Dominican Replublic", "🇩🇴"), ("Bridgetown", "Barbados", "🇧🇧"),
        ("Suva", "Fiji", "🇫🇯"), ("Port Moresby", "Papua New Guinea", "🇵🇬"), ("Ulaanbaatar", "Mongolia", "🇲🇳")

    ];
    // This creates a local variable `rng`
    let mut rng = thread_rng();

    //let idx = rand::thread_rng().gen_range(0..cities.len());
    let idx = rng.gen_range(0..cities.len());
    cities[idx]
}


