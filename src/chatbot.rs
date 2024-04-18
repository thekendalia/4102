use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use reqwest::Client;
use serde::Deserialize;
use serenity::async_trait;
use serenity::client::{Context as SContext, EventHandler};
use poise::Context as PoiseContext;
use serenity::model::gateway::Ready;
use tokio;

#[derive(Deserialize)]
pub struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    index: u64,
    pub message: Message,
    finish_reason: String,
}

#[derive(Deserialize)]
pub struct Message {
    role: String,
    pub content: String,
}

type Data = ();
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = PoiseContext<'a, Data, Error>;

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
    let response: OpenAIResponse = serde_json::from_str(&response_text)?;

    let response_text = response
        .choices
        .get(0)
        .and_then(|c| Some(c.message.content.clone()))
        .unwrap_or_else(|| String::from("No response"));

    poise::say_reply(ctx, response_text).await?;

    Ok(())
}