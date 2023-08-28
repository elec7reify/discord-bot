use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Color;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
slash_command,
prefix_command,
required_permissions = "SEND_MESSAGES"
)]
async fn builder(
    ctx: Context<'_>,
    #[description = "Enter a Title"] title: String,
    #[description = "Type a Description"] description: String
) -> Result<(), Error> {
    ctx.channel_id().send_message(ctx, |m| {
        m.embed(|e| e.description(description)
            .title(title).color(Color::FOOYOO)
        )
    }).await?;

    Ok(())
}


#[tokio::main]
async fn main() {
    let token = String::from(""); // Enter a token

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![builder()],
            ..Default::default()
        })
        .token(&token)
        .intents(serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}