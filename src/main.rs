use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Context as SContext, EventHandler};
use serenity::model::gateway::Ready;
use poise::serenity_prelude as serenity;
mod weather;
use weather::get_weather;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: SContext, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn weather(
    ctx: Context<'_>,
    #[description = "City to check weather for"] city: Option<String>,
) -> Result<(), Error> {
    let city = city // If the user didn't provide a city, default to "San Francisco"
        .as_deref()
        .unwrap_or("Charlotte");
    print!("City: {}", city);
    let weather = get_weather(city).await?;
    print!("{:?}", weather);
    let response = format!("The weather in {} is:\n{:?}", weather.name, 
        weather.main);
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
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

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}