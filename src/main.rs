#![allow(unused_imports)]
#![allow(dead_code)]

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::Context as PoiseContext;
use reqwest::Client;
use serde::Deserialize;
use serenity::async_trait;
use serenity::client::{Context as SContext, EventHandler};
use serenity::model::gateway::Ready;
mod chatbot;
mod weather;
use chrono::prelude::*;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use substring::Substring;
use tokio;
use weather::get_weather;

// Boilerplate from Poise docs
type Data = ();
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = PoiseContext<'a, Data, Error>;

// Serenity Event Handler
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // Ready event fired on client start-up
    async fn ready(&self, _: SContext, ready: Ready) {
        // Print bot username
        println!("{} is connected!", ready.user.name);
    }
}

// Pose command macro to create a slash command
#[poise::command(slash_command, prefix_command)]
async fn weather(
    ctx: Context<'_>,
    // Define optional City argument
    #[description = "City to check weather for"] city: Option<String>,
) -> Result<(), Error> {
    // Default to Charlotte if no city is provided
    let city = city // If the user didn't provide a city, default to "Charlotte"
        .as_deref()
        .unwrap_or("Charlotte");

    // Get weather data from our weather API
    match get_weather(city).await {
        Ok(weather) => {
            let fahrenheit = (weather.main.temp - 273.15) * (9. / 5.) + 32.;
            let fahrenheit_feels_like = (weather.main.feels_like - 273.15) * (9. / 5.) + 32.;
            let fahrenheit_temp_min = (weather.main.temp_min - 273.15) * (9. / 5.) + 32.;
            let fahrenheit_temp_max = (weather.main.temp_max - 273.15) * (9. / 5.) + 32.;
            let humidity_merc = weather.main.pressure as f64 * 0.02953;

            // Format the response as a string
            let response = format!(
                "The weather in {} is:\n🌡️ Temp: {:.2}°F  😓 Feels Like: {:.2}°F,\n🧊 Min Temp: {:.2}°F  🔥 Max Temp: {:.2}°F\n🌬️ Pressure: {:.2}inHg  💧 Humidity: {:.2}%",
                weather.name,
                fahrenheit,
                fahrenheit_feels_like,
                fahrenheit_temp_min,
                fahrenheit_temp_max,
                humidity_merc,
                weather.main.humidity
            );

            // Send formatted response to Discord
            ctx.say(response).await?;
        }
        Err(e) => {
            println!("Error: {:?}", e);
            let response = format!("The city '{}' doesn't exist or couldn't be found.", city);
            // Send error response here
            ctx.say(response).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn temp(
    ctx: Context<'_>,
    #[description = "City to check temperature for"] city: Option<String>,
) -> Result<(), Error> {
    // Default to Charlotte if no city is provided
    let city = city // If the user didn't provide a city, default to "Charlotte"
        .as_deref()
        .unwrap_or("Charlotte");

    // Call the get_weather function to fetch weather data for the specified city
    match get_weather(city).await {
        Ok(weather_response) => {
            // Extract temperature from the weather response in Kelvin
            let temperature_kelvin = weather_response.main.temp;

            // Convert temperature from Kelvin to Fahrenheit
            let temperature_fahrenheit = (temperature_kelvin - 273.15) * (9.0 / 5.0) + 32.0;

            // Convert temperature from Kelvin to Celsius
            let temperature_celsius = temperature_kelvin - 273.15;

            // Format the response with temperatures in all three units
            let response = format!(
                "The temperature in {} is:\n🌡️ {:.2}°K (Kelvin)\n🌡️ {:.2}°C (Celsius)\n🌡️ {:.2}°F (Fahrenheit)",
                city,
                temperature_kelvin,
                temperature_celsius,
                temperature_fahrenheit
            );

            // Send the response to the Discord channel
            ctx.say(response).await?;
        }
        Err(_) => {
            // Handle error if the city is not found or weather data cannot be retrieved
            let response = format!("Could not find temperature data for '{}'", city);
            ctx.say(response).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn sun(
    ctx: Context<'_>,
    #[description = "City to check temperature for"] city: Option<String>,
) -> Result<(), Error> {
    // Default to Charlotte if no city is provided
    let city = city // If the user didn't provide a city, default to "Charlotte"
        .as_deref()
        .unwrap_or("Charlotte");

    // Call the get_weather function to fetch weather data for the specified city
    match get_weather(city).await {
        Ok(weather_response) => {
            // Extract temperature from the weather response in Kelvin
            let sunset = weather_response.sys.sunset as i64;
            let sunrise = weather_response.sys.sunrise as i64;
            let sunset_utc = Utc.timestamp_opt(sunset, 0).unwrap().naive_utc();
            let sunrise_utc = Utc.timestamp_opt(sunrise, 0).unwrap().naive_utc();
            let dt = Local::now();
            let offset = dt.offset().clone();
            let sunset_local = DateTime::<Local>::from_naive_utc_and_offset(sunset_utc, offset);
            let sunrise_local = DateTime::<Local>::from_naive_utc_and_offset(sunrise_utc, offset);

            // Format the response as a string with UTC date and time
            let response = format!(
                "The sunset/sunrise in {} is:\nSunrise🌅 {:?}\nSunset🌙 {:?}",
                city, sunrise_local, sunset_local,
            );

            // Send the response to the Discord channel
            ctx.say(response).await?;
        }
        Err(_) => {
            // Handle error if the city is not found or weather data cannot be retrieved
            let response = format!("Could not find sunset/sunrise data for '{}'", city);
            ctx.say(response).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn clouds(
    ctx: Context<'_>,
    #[description = "City to check cloud coverage for"] city: Option<String>,
) -> Result<(), Error> {
    // Default to "Charlotte" if no city is provided
    let city = city.as_deref().unwrap_or("Charlotte");

    // Call the get_weather function to fetch weather data for the specified city
    match get_weather(city).await {
        Ok(weather_response) => {
            // Extract cloud coverage information from the weather response
            let cloud_coverage_percentage = weather_response.clouds.all;

            // Format the response with the cloud coverage percentage
            let response = format!(
                "The cloud coverage in {} is\n☁️ {:.0}%",
                city, cloud_coverage_percentage
            );

            // Send the response to the Discord channel
            ctx.say(response).await?;
        }
        Err(_) => {
            // Handle error if the city is not found or weather data cannot be retrieved
            let response = format!("Could not retrieve cloud information for '{}'.", city);
            ctx.say(response).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn wind(
    ctx: Context<'_>,
    #[description = "City to check wind speed for"] city: Option<String>,
) -> Result<(), Error> {
    // Default to "Charlotte" if no city is provided
    let city = city.as_deref().unwrap_or("Charlotte");

    // Call the get_weather function to fetch weather data for the specified city
    match get_weather(city).await {
        Ok(weather_response) => {
            // Extract wind speed information from the weather response
            let wind_speed_meters_per_sec = weather_response.wind.get_speed_meters_per_sec();
            let wind_speed_mph = weather_response.wind.get_speed_mph();

            // Format the response with the wind speed in miles per hour
            let response = format!(
                "The wind speed in {} is\n💨 {:.2} mph ({} m/s)",
                city, wind_speed_mph, wind_speed_meters_per_sec
            );

            // Send the response to the Discord channel
            ctx.say(response).await?;
        }
        Err(_) => {
            // Handle error if the city is not found or weather data cannot be retrieved
            let response = format!("Could not retrieve wind speed information for '{}'.", city);
            ctx.say(response).await?;
        }
    }

    Ok(())
}
#[poise::command(slash_command, prefix_command)]
pub async fn weatherfact(
    ctx: Context<'_>,
    #[description = "City to check wind speed for"] city: Option<String>,
) -> Result<(), Error> {
    let city = city.as_deref().unwrap_or("Charlotte");

    let http_client = Client::new();
    let api = std::env::var("OPENAI_API_KEY").expect("missing OPENAI_API_KEY");
    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": format!("what is a different crazy historical weather fact for the city of {} in only 2 sentences", city)
            }
        ],
        "max_tokens": 100,
    });

    let res = http_client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api)
        .json(&request_body)
        .send()
        .await?;

    let response_text = res.text().await?;
    let response: chatbot::OpenAIResponse = serde_json::from_str(&response_text)?;

    let response_text = response
        .choices
        .get(0)
        .and_then(|c| Some(c.message.content.clone()))
        .unwrap_or_else(|| String::from("No response"));

    poise::say_reply(ctx, response_text).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn weather_joke(ctx: Context<'_>) -> Result<(), Error> {
    let http_client = Client::new();
    let api = std::env::var("OPENAI_API_KEY").expect("missing OPENAI_API_KEY");
    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": "tell me a joke about the weather or jokes about meteorologists"
            }
        ],
        "max_tokens": 50,
    });

    let res = http_client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api)
        .json(&request_body)
        .send()
        .await?;

    let response_text = res.text().await?;
    let response: chatbot::OpenAIResponse = serde_json::from_str(&response_text)?;

    let response_text = response
        .choices
        .get(0)
        .and_then(|c| Some(c.message.content.clone()))
        .unwrap_or_else(|| String::from("No response"));

    poise::say_reply(ctx, response_text).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn random(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let (city, country, flag) = weather::get_random_city();  // Get a random city name

    match weather::get_weather(city).await {
        Ok(weather_response) => {
            let fahrenheit = (weather_response.main.temp - 273.15) * 9.0 / 5.0 + 32.0;
            let feels_like = (weather_response.main.feels_like - 273.15) * 9.0 / 5.0 + 32.0;
            let temp_min = (weather_response.main.temp_min - 273.15) * 9.0 / 5.0 + 32.0;
            let temp_max = (weather_response.main.temp_max - 273.15) * 9.0 / 5.0 + 32.0;
            let pressure = weather_response.main.pressure as f64 * 0.02953;
            
            let response = format!(
                "The weather in {}, {} {} is:\n🌡️ Temp: {:.2}°F  😓 Feels Like: {:.2}°F,\n🧊 Min Temp: {:.2}°F  🔥 Max Temp: {:.2}°F\n🌬️ Pressure: {:.2}inHg  💧 Humidity: {}%",
                city, country, flag, fahrenheit, feels_like, temp_min, temp_max, pressure, weather_response.main.humidity
            );
 
            ctx.say(response).await?;
        },
        Err(_) => {
            ctx.say(format!("Could not find weather data for '{}', '{}'", city, country)).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn distance(
    ctx: Context<'_>,
    #[description = "First city"] city1: Option<String>,
    #[description = "Second city"] city2: Option<String>,
) -> Result<(), Error> {
    let city1 = city1.as_deref().unwrap_or("Charlotte");
    let city2 = city2.as_deref().unwrap_or("Charlotte"); 

    pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

    match get_weather(city1).await {
        Ok(weather_response) => { 
            let coord1 = weather_response.coord.lat;
            let coord2 = weather_response.coord.lon;
            let _response = format!(
                "The weather in {}, lat {}, lon {}",
                city1, coord1, coord2
            );

            // ctx.say(response).await?;
        

        match get_weather(city2).await {
            Ok(weather_response) => { 
    
                let coord3 = weather_response.coord.lat;
                let coord4 = weather_response.coord.lon;

                use std::f64::consts::PI;

            fn haversine_distance(coord1: f64, coord2: f64, coord3: f64, coord4: f64) -> f64 {
                let r = 6371.0; // Earth's radius in kilometers
                let d_lat = (coord3 - coord1) * (PI / 180.0);
                let d_lon = (coord4 - coord2) * (PI / 180.0);
                let a = (d_lat / 2.0).sin().powi(2)
                        + (coord1 * (PI / 180.0)).cos()
                        * (coord3 * (PI / 180.0)).cos()
                        * (d_lon / 2.0).sin().powi(2);
                let c = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
                let d = r * c;
    
                d // Return the distance
            }
                let distance = haversine_distance(coord1, coord2, coord3, coord4);
                let miles = distance * 0.621371;
                let _response = format!(
                    "The distance between {} and {} is {:.2} kilometers, and {:.2} miles.",
                    city1, city2, distance, miles,
                );
    
                ctx.say(_response).await?;
            },
            Err(_) => {
                ctx.say(format!("Could not find weather data for '{}'", city2)).await?;
            }
        }
        },
        Err(_) => {
            ctx.say(format!("Could not find weather data for '{}'", city1)).await?;
        }
    
    }

    Ok(())
}

// Async main function
#[tokio::main]
async fn main() {
    // Load .env file and env vars
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // Create Poise framework with weather slash command
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // (Adding slash commands here)
            commands: vec![
                weather(),
                weather_joke(),
                temp(),
                clouds(),
                wind(),
                sun(),
                weatherfact(),
                random(),
                distance(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

    // Create Serenity client with Poise framework and event handler
    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;

    // Start the client
    client.unwrap().start().await.unwrap();
}
