use serenity::model::channel::Message;
use serenity::utils::Colour;
use serenity::builder::CreateEmbed;
use crate::weather::WeatherResponse;

pub async fn send_weather_embed(
    message: &Message,
    city: &str,
    weather_response: &WeatherResponse,
) -> Result<(), serenity::Error> {
    // Extract relevant weather information
    let temperature = weather_response.main.temp;
    let humidity = weather_response.main.humidity;

    // Create embed
    let embed = CreateEmbed::default()
        .title("Weather Report")
        .field("City", city, true)
        .field("Temperature (°C)", format!("{:.2}", temperature), true)
        .field("Humidity (%)", format!("{}", humidity), true)
        .color(Colour::BLITZ_BLUE)
        .footer(|f| f.text("Weather Bot"));

    // Send embed
    message.channel_id.send_message(|m| m.set_embed(embed)).await?;

    Ok(())
}