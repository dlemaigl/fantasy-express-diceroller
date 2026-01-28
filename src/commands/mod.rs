//! Discord slash commands for Fantasy Express dice roller

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::dice::{resolve_feat, roll_2d10_open, roll_with_modifier};
use crate::dice::roll::format_roll;
use crate::dice::feat::format_feat_result;

/// Handle the /roll command - basic 2d10 open-ended roll with optional modifier
pub async fn handle_roll(ctx: &Context, command: &CommandInteraction) {
    // Check for optional modifier
    let modifier: i32 = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "modifier")
        .and_then(|opt| opt.value.as_i64())
        .map(|v| v as i32)
        .unwrap_or(0);

    let result = roll_2d10_open();
    let final_total = result.raw_total + modifier;
    let formatted = format_roll(&result, if modifier != 0 { Some(modifier) } else { None });

    let response = if modifier != 0 {
        format!(
            "🎲 **Roll**\n━━━━━━━━━━━━━━\nDice: {}\n━━━━━━━━━━━━━━\n**Total: {}**",
            formatted, final_total
        )
    } else {
        format!(
            "🎲 **Roll**\n━━━━━━━━━━━━━━\nDice: {}\n━━━━━━━━━━━━━━\n**Total: {}**",
            formatted, result.raw_total
        )
    };

    send_response(ctx, command, &response).await;
}

/// Handle the /skill command - skill check with bonus and difficulty
pub async fn handle_skill(ctx: &Context, command: &CommandInteraction) {
    let mut bonus: i32 = 0;
    let mut difficulty: i32 = 0;

    for option in &command.data.options {
        match option.name.as_str() {
            "bonus" => {
                if let Some(val) = option.value.as_i64() {
                    bonus = val as i32;
                }
            }
            "difficulty" => {
                if let Some(val) = option.value.as_i64() {
                    difficulty = val as i32;
                }
            }
            _ => {}
        }
    }

    let total_modifier = bonus + difficulty;
    let (result, final_total) = roll_with_modifier(total_modifier);
    let feat_result = resolve_feat(final_total, result.is_fumble);

    let formatted_roll = format_roll(&result, Some(total_modifier));
    let formatted_feat = format_feat_result(&feat_result, final_total);

    let difficulty_str = if difficulty >= 0 {
        format!("+{}", difficulty)
    } else {
        format!("{}", difficulty)
    };

    let response = format!(
        "🎲 **Skill Roll**\n━━━━━━━━━━━━━━\nDice: {}\nBonus: {:+} | Difficulty: {}\n━━━━━━━━━━━━━━\n{}",
        formatted_roll, bonus, difficulty_str, formatted_feat
    );

    send_response(ctx, command, &response).await;
}

/// Register slash commands with Discord
pub fn register_commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("roll")
            .description("Roll 2d10 with open-ended explosions (19-20 explode)")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "modifier",
                    "Optional bonus/penalty to add (e.g. +15 for attack bonus)",
                )
                .required(false)
                .max_int_value(100),
            ),
        CreateCommand::new("skill")
            .description("Make a skill check with bonus and difficulty")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "bonus",
                    "Your total skill bonus (stat + ranks + modifiers)",
                )
                .required(true)
                .min_int_value(0)
                .max_int_value(100),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "difficulty",
                    "Difficulty modifier (Easy +4, Normal 0, Hard -6, Heroic -12, etc.)",
                )
                .required(false)
                .min_int_value(0)
                .max_int_value(20),
            ),
    ]
}

/// Send a response to a slash command interaction
async fn send_response(ctx: &Context, command: &CommandInteraction, content: &str) {
    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content(content),
    );

    if let Err(e) = command.create_response(&ctx.http, response).await {
        eprintln!("Error sending response: {:?}", e);
    }
}
