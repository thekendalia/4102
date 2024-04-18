use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use serenity::async_trait;
use serenity::client::{Context as SContext, EventHandler};
use serenity::model::gateway::Ready;
mod weather;
use weather::get_weather;

// Boilerplate from Poise docs
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
        },
        Err(_) => {
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
            commands: vec![weather(), temp(), clouds(), wind()], 
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
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

