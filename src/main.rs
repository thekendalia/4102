use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Context as SContext, EventHandler};
use serenity::model::gateway::Ready;
use poise::serenity_prelude as serenity;
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
    let city = city // If the user didn't provide a city, default to "San Francisco"
        .as_deref()
        .unwrap_or("Charlotte");

    // Get weather data from our weather API 
    let weather = get_weather(city).await?;

    // Format the response as a string
    let response = format!("The weather in {} is:\n{:?}", weather.name, 
        weather.main);

    // Send formatted response to Discord
    ctx.say(response).await?;
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
            commands: vec![weather()],
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