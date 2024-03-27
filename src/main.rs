mod weather;
use weather::get_weather;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut line = String::new();
    println!("Enter a city :");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("City: {}", line);
    println!("Please wait...");

    let weather_response = get_weather(&line).await?;

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