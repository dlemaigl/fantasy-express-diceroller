//! Discord slash commands for Fantasy Express dice roller

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::dice::{resolve_feat, roll_2d10_closed, roll_2d10_open, roll_with_modifier};
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

/// Handle the /initiative command - 2d10 (NOT open-ended) + Dex + modifiers
pub async fn handle_initiative(ctx: &Context, command: &CommandInteraction) {
    let mut dexterity: i32 = 0;
    let mut modifier: i32 = 0;

    for option in &command.data.options {
        match option.name.as_str() {
            "dexterity" => {
                if let Some(val) = option.value.as_i64() {
                    dexterity = val as i32;
                }
            }
            "modifier" => {
                if let Some(val) = option.value.as_i64() {
                    modifier = val as i32;
                }
            }
            _ => {}
        }
    }

    let (d1, d2, dice_total) = roll_2d10_closed();
    let final_total = dice_total + dexterity + modifier;

    let modifier_str = if modifier != 0 {
        format!(" {:+}", modifier)
    } else {
        String::new()
    };

    let response = format!(
        "⚔️ **Initiative**\n━━━━━━━━━━━━━━\nDice: [{}, {}] = {}\nDexterity: {:+}{}\n━━━━━━━━━━━━━━\n**Initiative: {}**",
        d1, d2, dice_total, dexterity, modifier_str, final_total
    );

    send_response(ctx, command, &response).await;
}

/// Handle the /attack command - attack roll with FEAT resolution
pub async fn handle_attack(ctx: &Context, command: &CommandInteraction) {
    let mut attack_bonus: i32 = 0;
    let mut defense: i32 = 0;
    let mut damage_rating: i32 = 0;

    for option in &command.data.options {
        match option.name.as_str() {
            "attack_bonus" => {
                if let Some(val) = option.value.as_i64() {
                    attack_bonus = val as i32;
                }
            }
            "defense" => {
                if let Some(val) = option.value.as_i64() {
                    defense = val as i32;
                }
            }
            "damage" => {
                if let Some(val) = option.value.as_i64() {
                    damage_rating = val as i32;
                }
            }
            _ => {}
        }
    }

    let total_modifier = attack_bonus - defense;
    let (result, final_total) = roll_with_modifier(total_modifier);
    let feat_result = resolve_feat(final_total, result.is_fumble);

    // Format dice display without modifier (we'll show it separately)
    let dice_display = if result.explosions.is_empty() {
        format!("[{}, {}] = {}", result.base_rolls.0, result.base_rolls.1, result.raw_total)
    } else {
        let mut s = format!("[{}, {}] = {}", result.base_rolls.0, result.base_rolls.1, 
            result.base_rolls.0 + result.base_rolls.1);
        s.push_str(" 💥 → ");
        for (i, exp) in result.explosions.iter().enumerate() {
            if i > 0 { s.push_str(" → "); }
            s.push_str(&format!("[{}]", exp));
            if *exp >= 19 { s.push_str(" 💥"); }
        }
        s.push_str(&format!(" = {}", result.raw_total));
        s
    };

    let formatted_feat = format_feat_result(&feat_result, final_total);

    // Calculate damage if hit (20+)
    let damage_info = if final_total >= 20 && damage_rating > 0 {
        let points_over_20 = (final_total - 20).max(0);
        let max_bonus = damage_rating * 2; // Cap at 3x DR total
        let hit_damage = damage_rating + points_over_20.min(max_bonus);
        format!("\n💥 **Damage: {}** (DR {} + {} bonus)", hit_damage, damage_rating, points_over_20.min(max_bonus))
    } else if final_total >= 20 {
        String::from("\n💥 Hit! Add DR + points over 20 (max 3x DR)")
    } else {
        String::new()
    };

    // Show clear breakdown: dice + AB - DM = total
    let response = format!(
        "⚔️ **Attack Roll**\n━━━━━━━━━━━━━━\nDice: {}\nCalc: {} {:+} (AB) - {} (DM) = **{}**\n━━━━━━━━━━━━━━\n{}{}",
        dice_display, result.raw_total, attack_bonus, defense, final_total, formatted_feat, damage_info
    );

    send_response(ctx, command, &response).await;
}

/// Handle the /save command - saving throw roll
pub async fn handle_save(ctx: &Context, command: &CommandInteraction) {
    let mut save_type = String::from("TSR");
    let mut bonus: i32 = 0;
    let mut attack_level: i32 = 0;

    for option in &command.data.options {
        match option.name.as_str() {
            "type" => {
                if let Some(val) = option.value.as_str() {
                    save_type = val.to_uppercase();
                }
            }
            "bonus" => {
                if let Some(val) = option.value.as_i64() {
                    bonus = val as i32;
                }
            }
            "attack_level" => {
                if let Some(val) = option.value.as_i64() {
                    attack_level = val as i32;
                }
            }
            _ => {}
        }
    }

    let save_name = match save_type.as_str() {
        "TSR" => "Toughness (TSR)",
        "RSR" => "Reflex (RSR)",
        "WSR" => "Will (WSR)",
        _ => "Save",
    };

    let total_modifier = bonus - attack_level;
    let (result, final_total) = roll_with_modifier(total_modifier);
    let feat_result = resolve_feat(final_total, result.is_fumble);

    let formatted_roll = format_roll(&result, Some(total_modifier));
    let formatted_feat = format_feat_result(&feat_result, final_total);

    let attack_level_str = if attack_level != 0 {
        format!(" | Attack Level: {}", attack_level)
    } else {
        String::new()
    };

    let response = format!(
        "🛡️ **{} Save**\n━━━━━━━━━━━━━━\nDice: {}\nBonus: {:+}{}\n━━━━━━━━━━━━━━\n{}",
        save_name, formatted_roll, bonus, attack_level_str, formatted_feat
    );

    send_response(ctx, command, &response).await;
}

/// Handle the /cast command - spell casting roll with Magical Resonance check
pub async fn handle_cast(ctx: &Context, command: &CommandInteraction) {
    let mut tier: i32 = 1;
    let mut bonus: i32 = 0;

    for option in &command.data.options {
        match option.name.as_str() {
            "tier" => {
                if let Some(val) = option.value.as_i64() {
                    tier = val as i32;
                }
            }
            "bonus" => {
                if let Some(val) = option.value.as_i64() {
                    bonus = val as i32;
                }
            }
            _ => {}
        }
    }

    let (result, final_total) = roll_with_modifier(bonus);
    let feat_result = resolve_feat(final_total, result.is_fumble);
    let formatted_roll = format_roll(&result, Some(bonus));
    let formatted_feat = format_feat_result(&feat_result, final_total);

    // Check for Magical Resonance: doubles where die value ≤ tier
    let d1 = result.base_rolls.0;
    let d2 = result.base_rolls.1;
    let resonance_warning = if d1 == d2 && (d1 as i32) <= tier {
        format!("\n\n⚡ **MAGICAL RESONANCE!** Rolled double {}s (≤ Tier {}). Roll on Resonance table!", d1, tier)
    } else {
        String::new()
    };

    let response = format!(
        "✨ **Casting Roll** (Tier {})\n━━━━━━━━━━━━━━\nDice: {}\nBonus: {:+}\n━━━━━━━━━━━━━━\n{}{}",
        tier, formatted_roll, bonus, formatted_feat, resonance_warning
    );

    send_response(ctx, command, &response).await;
}

/// Handle the /fumble command - weapon fumble table roll
pub async fn handle_fumble(ctx: &Context, command: &CommandInteraction) {
    let mut weapon_mod: i32 = 0;
    let mut skill_ranks: i32 = 0;
    let mut weapon_type = String::from("Brawl/Hand");

    for option in &command.data.options {
        match option.name.as_str() {
            "weapon_type" => {
                if let Some(val) = option.value.as_str() {
                    weapon_type = val.to_string();
                    weapon_mod = match val {
                        "brawl" => 0,
                        "short_edged" => 2,
                        "long_edged" => 4,
                        "two_handed" => 6,
                        "polearm" => 10,
                        _ => 0,
                    };
                }
            }
            "skill_ranks" => {
                if let Some(val) = option.value.as_i64() {
                    skill_ranks = val as i32;
                }
            }
            _ => {}
        }
    }

    let rank_reduction = skill_ranks / 2;
    let total_mod = weapon_mod - rank_reduction;
    let (d1, d2, dice_total) = roll_2d10_closed();
    let final_total = dice_total + total_mod;

    let fumble_result = match final_total {
        t if t <= 15 => "📋 Make Assessment Roll next Upkeep",
        16..=20 => "📋 Assessment + choose 1: Drop weapon OR Take 1 SL critical to self",
        21..=25 => "📋 Assessment + choose 2: Drop, 2 SL critical to self, Hit ally",
        26..=30 => "📋 Assessment + choose 2: Drop, 2 SL critical, Hit ally, Weapon breaks",
        _ => "💀 Assessment + choose 2: Drop, 3 SL critical, Hit ally, Weapon breaks",
    };

    let response = format!(
        "⚠️ **Weapon Fumble**\n━━━━━━━━━━━━━━\nDice: [{}, {}] = {}\nWeapon: {} ({:+}) | Skill ranks: {} ({:+})\n━━━━━━━━━━━━━━\n**Total: {}**\n{}",
        d1, d2, dice_total, weapon_type, weapon_mod, skill_ranks, -rank_reduction, final_total, fumble_result
    );

    send_response(ctx, command, &response).await;
}

/// Handle the /spellfumble command - spell fumble table roll
pub async fn handle_spellfumble(ctx: &Context, command: &CommandInteraction) {
    let mut tier: i32 = 1;
    let mut ranks: i32 = 0;
    let mut spell_mod: i32 = 0;
    let mut spell_type = String::from("Utility");

    for option in &command.data.options {
        match option.name.as_str() {
            "tier" => {
                if let Some(val) = option.value.as_i64() {
                    tier = val as i32;
                }
            }
            "ranks" => {
                if let Some(val) = option.value.as_i64() {
                    ranks = val as i32;
                }
            }
            "spell_type" => {
                if let Some(val) = option.value.as_str() {
                    spell_type = val.to_string();
                    spell_mod = match val {
                        "healing" => 0,
                        "utility" => 2,
                        "enchantment" => 4,
                        "alteration" => 6,
                        "dark" => 6,
                        _ => 0,
                    };
                }
            }
            _ => {}
        }
    }

    let total_mod = tier + spell_mod - ranks;
    let (d1, d2, dice_total) = roll_2d10_closed();
    let final_total = dice_total + total_mod;

    let fumble_result = match final_total {
        t if t <= 18 => "💫 Lose Mana Points",
        19..=22 => "💫 Lose Mana, Stunned 1 round",
        23..=26 => "💫 Stunned 1 round + choose 1: Lose double Mana OR Spell delays 2 rounds",
        27..=30 => "💫 Stunned 2 rounds + choose 1: Lose double Mana OR Spell hits different target",
        31..=34 => "⚡ Stunned 3 rounds + choose 2: Lose triple Mana, wrong target, Magical Resonance (+4)",
        _ => "💀 Stunned 4 rounds + choose 2: Lose triple Mana + Weary, wrong target, Resonance (+10), KO 6h",
    };

    let response = format!(
        "💥 **Spell Fumble**\n━━━━━━━━━━━━━━\nDice: [{}, {}] = {}\nTier: {:+} | Type: {} ({:+}) | Ranks: {:+}\n━━━━━━━━━━━━━━\n**Total: {}**\n{}",
        d1, d2, dice_total, tier, spell_type, spell_mod, -ranks, final_total, fumble_result
    );

    send_response(ctx, command, &response).await;
}

/// Handle the /help command - show available commands
pub async fn handle_help(ctx: &Context, command: &CommandInteraction) {
    let response = "📖 **Fantasy Express Commands**\n━━━━━━━━━━━━━━\n\
        **Basic Rolls**\n\
        `/roll` - 2d10 open-ended (explodes on 19-20)\n\
        `/skill` - Skill check with FEAT resolution\n\n\
        **Combat**\n\
        `/initiative` - Initiative (2d10 NOT open-ended)\n\
        `/attack` - Attack roll with damage calc\n\
        `/save` - Saving throw (TSR/RSR/WSR)\n\n\
        **Magic**\n\
        `/cast` - Spell casting with Resonance check\n\n\
        **Fumbles**\n\
        `/fumble` - Weapon fumble table\n\
        `/spellfumble` - Spell fumble table\n\n\
        **Reference**\n\
        `/difficulty` - Difficulty modifiers\n\
        `/opposed` - Opposed roll (two participants)";

    send_response(ctx, command, response).await;
}

/// Handle the /difficulty command - show difficulty reference
pub async fn handle_difficulty(ctx: &Context, command: &CommandInteraction) {
    let response = "📊 **Difficulty Modifiers**\n━━━━━━━━━━━━━━\n\
        | Difficulty | Modifier |\n\
        |------------|----------|\n\
        | Easy | +4 |\n\
        | Normal | +0 |\n\
        | Challenging | -3 |\n\
        | Hard | -6 |\n\
        | Very Hard | -9 |\n\
        | Heroic | -12 |\n\
        | Legendary | -15 |\n\
        | Mythic | -20 |\n\n\
        *Taking the Time: +4 (double time)*";

    send_response(ctx, command, response).await;
}

/// Handle the /opposed command - opposed roll between two participants
pub async fn handle_opposed(ctx: &Context, command: &CommandInteraction) {
    let mut bonus1: i32 = 0;
    let mut bonus2: i32 = 0;

    for option in &command.data.options {
        match option.name.as_str() {
            "bonus1" => {
                if let Some(val) = option.value.as_i64() {
                    bonus1 = val as i32;
                }
            }
            "bonus2" => {
                if let Some(val) = option.value.as_i64() {
                    bonus2 = val as i32;
                }
            }
            _ => {}
        }
    }

    let (result1, total1) = roll_with_modifier(bonus1);
    let (result2, total2) = roll_with_modifier(bonus2);

    let formatted1 = format_roll(&result1, Some(bonus1));
    let formatted2 = format_roll(&result2, Some(bonus2));

    let winner = if result1.is_fumble && result2.is_fumble {
        "💀 Both fumbled!"
    } else if result1.is_fumble {
        "💀 Participant 1 fumbled! **Participant 2 wins**"
    } else if result2.is_fumble {
        "💀 Participant 2 fumbled! **Participant 1 wins**"
    } else if total1 > total2 {
        "🏆 **Participant 1 wins!**"
    } else if total2 > total1 {
        "🏆 **Participant 2 wins!**"
    } else {
        "⚔️ **Tie!** Neither wins, re-roll or find another solution"
    };

    let response = format!(
        "⚔️ **Opposed Roll**\n━━━━━━━━━━━━━━\n👤 **P1**: {} = **{}**\n👤 **P2**: {} = **{}**\n━━━━━━━━━━━━━━\n{}",
        formatted1, total1, formatted2, total2, winner
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
            ),
        // Initiative command
        CreateCommand::new("initiative")
            .description("Roll initiative: 2d10 (not open-ended) + Dexterity + modifiers")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "dexterity",
                    "Your Dexterity stat",
                )
                .required(true)
                .max_int_value(20),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "modifier",
                    "Additional modifiers (weapon, encumbrance, wounded, etc.)",
                )
                .required(false),
            ),
        // Attack command
        CreateCommand::new("attack")
            .description("Make an attack roll with FEAT resolution")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "attack_bonus",
                    "Your total Attack Bonus (skill + weapon + modifiers)",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "defense",
                    "Target's Defensive Modifier (DM)",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "damage",
                    "Weapon Damage Rating (DR) for damage calculation",
                )
                .required(false)
                .min_int_value(1)
                .max_int_value(50),
            ),
        // Save command
        CreateCommand::new("save")
            .description("Make a saving throw (TSR, RSR, or WSR)")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "type",
                    "Type of save: TSR (Toughness), RSR (Reflex), WSR (Will)",
                )
                .required(true)
                .add_string_choice("Toughness (TSR)", "TSR")
                .add_string_choice("Reflex (RSR)", "RSR")
                .add_string_choice("Will (WSR)", "WSR"),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "bonus",
                    "Your total save bonus (7 + Stat + Kin + Level + mods)",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "attack_level",
                    "Attack Level/Tier of the effect you're saving against",
                )
                .required(false)
                .min_int_value(0)
                .max_int_value(20),
            ),
        // Cast command
        CreateCommand::new("cast")
            .description("Cast a spell with Magical Resonance detection")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "tier",
                    "Spell Tier (1-10)",
                )
                .required(true)
                .min_int_value(1)
                .max_int_value(10),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "bonus",
                    "Your total casting bonus (skill + modifiers)",
                )
                .required(true),
            ),
        // Fumble command
        CreateCommand::new("fumble")
            .description("Roll on the weapon fumble table")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "weapon_type",
                    "Type of weapon",
                )
                .required(true)
                .add_string_choice("Brawl/Hand/Short impact (+0)", "brawl")
                .add_string_choice("Short edged/Long impact (+2)", "short_edged")
                .add_string_choice("Long edged (+4)", "long_edged")
                .add_string_choice("Two-handed/Chain (+6)", "two_handed")
                .add_string_choice("Polearm/Net/Whip (+10)", "polearm"),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "skill_ranks",
                    "Your ranks in the weapon skill (-1 per 2 ranks)",
                )
                .required(false)
                .min_int_value(0)
                .max_int_value(20),
            ),
        // Spell Fumble command
        CreateCommand::new("spellfumble")
            .description("Roll on the spell fumble table")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "tier",
                    "Spell Tier",
                )
                .required(true)
                .min_int_value(1)
                .max_int_value(10),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "ranks",
                    "Your ranks in the spell skill",
                )
                .required(false)
                .min_int_value(0)
                .max_int_value(20),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "spell_type",
                    "Type of spell",
                )
                .required(false)
                .add_string_choice("Healing/Divination (+0)", "healing")
                .add_string_choice("Utility/Defensive (+2)", "utility")
                .add_string_choice("Enchantment (+4)", "enchantment")
                .add_string_choice("Alteration (+6)", "alteration")
                .add_string_choice("Dark/Elemental (+6)", "dark"),
            ),
        // Help command
        CreateCommand::new("help")
            .description("Show all available commands"),
        // Difficulty command
        CreateCommand::new("difficulty")
            .description("Show difficulty modifier reference table"),
        // Opposed roll command
        CreateCommand::new("opposed")
            .description("Make an opposed roll between two participants")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "bonus1",
                    "Participant 1's bonus",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "bonus2",
                    "Participant 2's bonus",
                )
                .required(true),
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
