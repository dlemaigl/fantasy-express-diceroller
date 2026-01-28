//! Core dice rolling mechanics for Fantasy Express RPG
//!
//! Implements 2d10 open-ended dice with:
//! - Explosion on 19-20 (roll again, add to total)
//! - Critical Failure (Fumble) on unmodified 2

use rand::Rng;

/// Result of a 2d10 open-ended roll
#[derive(Debug, Clone)]
pub struct DiceResult {
    /// The two initial d10 rolls
    pub base_rolls: (u8, u8),
    /// Chain of explosion rolls (if any) - each is (d1, d2)
    pub explosions: Vec<(u8, u8)>,
    /// Raw total before modifiers
    pub raw_total: i32,
    /// Whether this is a fumble (unmodified 2)
    pub is_fumble: bool,
    /// Whether an explosion occurred
    #[allow(dead_code)]
    pub exploded: bool,
}

impl DiceResult {
    /// Get the unmodified sum of the base dice (used for fumble/explosion checks)
    pub fn unmodified_sum(&self) -> u8 {
        self.base_rolls.0 + self.base_rolls.1
    }
}

/// Roll a single d10 (1-10)
fn roll_d10() -> u8 {
    rand::thread_rng().gen_range(1..=10)
}

/// Roll 2d10 with open-ended explosions
///
/// # Rules
/// - Roll 2d10 and sum
/// - If unmodified total is 19 or 20, roll again and add (chain explosions)
/// - If unmodified total is 2 (both dice show 1), it's a fumble
pub fn roll_2d10_open() -> DiceResult {
    let d1 = roll_d10();
    let d2 = roll_d10();
    let base_sum = d1 + d2;

    let is_fumble = d1 == 1 && d2 == 1;
    let mut explosions = Vec::new();
    let mut total = base_sum as i32;

    // Check for explosion (19 or 20)
    if base_sum >= 19 {
        let mut last_roll = base_sum;
        while last_roll >= 19 {
            let exp_d1 = roll_d10();
            let exp_d2 = roll_d10();
            let explosion_sum = exp_d1 + exp_d2;
            explosions.push((exp_d1, exp_d2));
            total += explosion_sum as i32;
            last_roll = explosion_sum;
        }
    }

    let exploded = !explosions.is_empty();

    DiceResult {
        base_rolls: (d1, d2),
        explosions,
        raw_total: total,
        is_fumble,
        exploded,
    }
}

/// Roll 2d10 open-ended with a modifier applied
pub fn roll_with_modifier(modifier: i32) -> (DiceResult, i32) {
    let result = roll_2d10_open();
    let final_total = result.raw_total + modifier;
    (result, final_total)
}

/// Roll 2d10 WITHOUT open-ended explosions (for initiative)
///
/// Unlike combat and skill rolls, initiative uses standard 2d10
/// without the explosion mechanic on 19-20.
pub fn roll_2d10_closed() -> (u8, u8, i32) {
    let d1 = roll_d10();
    let d2 = roll_d10();
    (d1, d2, (d1 + d2) as i32)
}

/// Format a dice result for display
pub fn format_roll(result: &DiceResult, modifier: Option<i32>) -> String {
    let mut output = format!(
        "[{}, {}] = {}",
        result.base_rolls.0, result.base_rolls.1, result.unmodified_sum()
    );

    if !result.explosions.is_empty() {
        output.push_str(" 💥 → ");
        for (i, (exp_d1, exp_d2)) in result.explosions.iter().enumerate() {
            if i > 0 {
                output.push_str(" → ");
            }
            let exp_sum = exp_d1 + exp_d2;
            output.push_str(&format!("[{}, {}] = {}", exp_d1, exp_d2, exp_sum));
            if exp_sum >= 19 {
                output.push_str(" 💥");
            }
        }
        output.push_str(&format!(" = {}", result.raw_total));
    }

    if let Some(mod_val) = modifier {
        let sign = if mod_val >= 0 { "+" } else { "" };
        let final_total = result.raw_total + mod_val;
        output.push_str(&format!(" {} {} = **{}**", sign, mod_val, final_total));
    }

    if result.is_fumble {
        output.push_str(" ⚠️ FUMBLE!");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d10_range() {
        for _ in 0..100 {
            let roll = roll_d10();
            assert!(roll >= 1 && roll <= 10, "d10 out of range: {}", roll);
        }
    }

    #[test]
    fn test_dice_result_unmodified_sum() {
        let result = DiceResult {
            base_rolls: (5, 7),
            explosions: vec![],
            raw_total: 12,
            is_fumble: false,
            exploded: false,
        };
        assert_eq!(result.unmodified_sum(), 12);
    }

    #[test]
    fn test_fumble_detection() {
        let result = DiceResult {
            base_rolls: (1, 1),
            explosions: vec![],
            raw_total: 2,
            is_fumble: true,
            exploded: false,
        };
        assert!(result.is_fumble);
        assert_eq!(result.unmodified_sum(), 2);
    }

    #[test]
    fn test_format_basic_roll() {
        let result = DiceResult {
            base_rolls: (6, 8),
            explosions: vec![],
            raw_total: 14,
            is_fumble: false,
            exploded: false,
        };
        let formatted = format_roll(&result, None);
        assert!(formatted.contains("[6, 8]"));
        assert!(formatted.contains("= 14"));
    }

    #[test]
    fn test_format_with_modifier() {
        let result = DiceResult {
            base_rolls: (6, 8),
            explosions: vec![],
            raw_total: 14,
            is_fumble: false,
            exploded: false,
        };
        let formatted = format_roll(&result, Some(5));
        assert!(formatted.contains("+ 5"));
        assert!(formatted.contains("**19**"));
    }

    #[test]
    fn test_format_explosion() {
        let result = DiceResult {
            base_rolls: (10, 10),
            explosions: vec![(8, 7)],  // [8, 7] = 15
            raw_total: 35,
            is_fumble: false,
            exploded: true,
        };
        let formatted = format_roll(&result, None);
        assert!(formatted.contains("💥"));
        assert!(formatted.contains("[8, 7] = 15"));
        assert!(formatted.contains("= 35"));
    }
}
