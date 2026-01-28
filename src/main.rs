//! Fantasy Express Discord Bot
//!
//! A dice roller bot for Fantasy Express RPG that handles:
//! - 2d10 open-ended rolls with explosion on 19-20
//! - FEAT table resolution for skill checks
//! - Fumble detection on unmodified 2

use std::env;

use serenity::all::{
    Client, Context, EventHandler, GatewayIntents, GuildId, Interaction, Ready,
};
use serenity::async_trait;

mod commands;
mod dice;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🎲 {} is connected!", ready.user.name);

        // Register commands globally (or to a specific guild for testing)
        let commands = commands::register_commands();

        // Check for a test guild ID to register commands faster during development
        if let Ok(guild_id) = env::var("TEST_GUILD_ID") {
            if let Ok(id) = guild_id.parse::<u64>() {
                let guild = GuildId::new(id);
                if let Err(e) = guild.set_commands(&ctx.http, commands).await {
                    eprintln!("Error registering guild commands: {:?}", e);
                } else {
                    println!("✅ Registered commands to test guild {}", id);
                }
            }
        } else {
            // Register globally (may take up to an hour to propagate)
            if let Err(e) = serenity::all::Command::set_global_commands(&ctx.http, commands).await
            {
                eprintln!("Error registering global commands: {:?}", e);
            } else {
                println!("✅ Registered global commands (may take up to 1 hour to appear)");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "roll" => commands::handle_roll(&ctx, &command).await,
                "skill" => commands::handle_skill(&ctx, &command).await,
                "initiative" => commands::handle_initiative(&ctx, &command).await,
                "attack" => commands::handle_attack(&ctx, &command).await,
                "save" => commands::handle_save(&ctx, &command).await,
                "cast" => commands::handle_cast(&ctx, &command).await,
                "fumble" => commands::handle_fumble(&ctx, &command).await,
                "spellfumble" => commands::handle_spellfumble(&ctx, &command).await,
                "help" => commands::handle_help(&ctx, &command).await,
                "difficulty" => commands::handle_difficulty(&ctx, &command).await,
                "opposed" => commands::handle_opposed(&ctx, &command).await,
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in environment or .env file");

    let intents = GatewayIntents::empty();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    println!("🚀 Starting Fantasy Express Bot...");

    if let Err(e) = client.start().await {
        eprintln!("Client error: {:?}", e);
    }
}
