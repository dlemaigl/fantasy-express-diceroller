//! FEAT (Fantasy Express Action/Task) table resolution
//!
//! All actions use the same target number: 20
//! Results determine success levels and special outcomes

/// Result of a FEAT check
#[derive(Debug, Clone, PartialEq)]
pub enum FeatResult {
    /// Unmodified roll of 2 - automatic failure with consequences
    CriticalFailure,
    /// Roll 3-15: Task not accomplished
    Failure,
    /// Roll 16-19: Bare minimum or partial success with cost
    PartialSuccess,
    /// Roll 20-34: Task accomplished with success levels
    Success { success_levels: u8 },
    /// Roll 35+: Best possible outcome, 5 success levels
    CriticalSuccess,
}

impl FeatResult {
    /// Get the number of success levels
    #[allow(dead_code)]
    pub fn success_levels(&self) -> u8 {
        match self {
            FeatResult::CriticalFailure => 0,
            FeatResult::Failure => 0,
            FeatResult::PartialSuccess => 0,
            FeatResult::Success { success_levels } => *success_levels,
            FeatResult::CriticalSuccess => 5,
        }
    }

    /// Check if this is any kind of success
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        matches!(
            self,
            FeatResult::Success { .. } | FeatResult::CriticalSuccess
        )
    }

    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            FeatResult::CriticalFailure => "💀",
            FeatResult::Failure => "❌",
            FeatResult::PartialSuccess => "⚠️",
            FeatResult::Success { .. } => "✅",
            FeatResult::CriticalSuccess => "🌟",
        }
    }

    /// Get display name
    pub fn name(&self) -> String {
        match self {
            FeatResult::CriticalFailure => "CRITICAL FAILURE".to_string(),
            FeatResult::Failure => "FAILURE".to_string(),
            FeatResult::PartialSuccess => "PARTIAL SUCCESS".to_string(),
            FeatResult::Success { success_levels } => format!("SUCCESS ({} SL)", success_levels),
            FeatResult::CriticalSuccess => "CRITICAL SUCCESS (5 SL)".to_string(),
        }
    }

    /// Get description of the result
    pub fn description(&self) -> &'static str {
        match self {
            FeatResult::CriticalFailure => {
                "Fumble! Danger, broken equipment, or +4 to enemies' next roll."
            }
            FeatResult::Failure => "Task not accomplished.",
            FeatResult::PartialSuccess => {
                "Bare minimum success with cost/complication, or failure."
            }
            FeatResult::Success { .. } => "Task accomplished! Spend Success Levels on Boons.",
            FeatResult::CriticalSuccess => {
                "Best possible outcome! 5 Success Levels for Boons."
            }
        }
    }
}

/// Resolve a FEAT check from a final total (after all modifiers)
///
/// # Arguments
/// * `total` - The final roll total after modifiers
/// * `is_fumble` - Whether the unmodified roll was 2 (critical failure)
///
/// # FEAT Table
/// | Roll Result | Outcome | Success Levels |
/// |-------------|---------|----------------|
/// | UM 2        | Critical Failure | Fumble Roll |
/// | 3-15        | Failure | 0 |
/// | 16-19       | Partial Success | 0 |
/// | 20-22       | Success | 1 SL |
/// | 23-26       | Success | 2 SL |
/// | 27-30       | Success | 3 SL |
/// | 31-34       | Success | 4 SL |
/// | 35+         | Critical Success | 5 SL |
pub fn resolve_feat(total: i32, is_fumble: bool) -> FeatResult {
    // Fumble always results in critical failure regardless of modifiers
    if is_fumble {
        return FeatResult::CriticalFailure;
    }

    match total {
        t if t <= 15 => FeatResult::Failure,
        16..=19 => FeatResult::PartialSuccess,
        20..=22 => FeatResult::Success { success_levels: 1 },
        23..=26 => FeatResult::Success { success_levels: 2 },
        27..=30 => FeatResult::Success { success_levels: 3 },
        31..=34 => FeatResult::Success { success_levels: 4 },
        _ => FeatResult::CriticalSuccess, // 35+
    }
}

/// Format a complete FEAT result for Discord display
pub fn format_feat_result(result: &FeatResult, total: i32) -> String {
    format!(
        "{} **{}** (Total: {})\n{}",
        result.emoji(),
        result.name(),
        total,
        result.description()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fumble_always_critical_failure() {
        // Even with high modifiers, fumble is critical failure
        assert_eq!(resolve_feat(50, true), FeatResult::CriticalFailure);
        assert_eq!(resolve_feat(2, true), FeatResult::CriticalFailure);
    }

    #[test]
    fn test_failure_range() {
        assert_eq!(resolve_feat(3, false), FeatResult::Failure);
        assert_eq!(resolve_feat(10, false), FeatResult::Failure);
        assert_eq!(resolve_feat(15, false), FeatResult::Failure);
    }

    #[test]
    fn test_partial_success_range() {
        assert_eq!(resolve_feat(16, false), FeatResult::PartialSuccess);
        assert_eq!(resolve_feat(17, false), FeatResult::PartialSuccess);
        assert_eq!(resolve_feat(19, false), FeatResult::PartialSuccess);
    }

    #[test]
    fn test_success_levels() {
        // 1 SL: 20-22
        assert_eq!(
            resolve_feat(20, false),
            FeatResult::Success { success_levels: 1 }
        );
        assert_eq!(
            resolve_feat(22, false),
            FeatResult::Success { success_levels: 1 }
        );

        // 2 SL: 23-26
        assert_eq!(
            resolve_feat(23, false),
            FeatResult::Success { success_levels: 2 }
        );
        assert_eq!(
            resolve_feat(26, false),
            FeatResult::Success { success_levels: 2 }
        );

        // 3 SL: 27-30
        assert_eq!(
            resolve_feat(27, false),
            FeatResult::Success { success_levels: 3 }
        );
        assert_eq!(
            resolve_feat(30, false),
            FeatResult::Success { success_levels: 3 }
        );

        // 4 SL: 31-34
        assert_eq!(
            resolve_feat(31, false),
            FeatResult::Success { success_levels: 4 }
        );
        assert_eq!(
            resolve_feat(34, false),
            FeatResult::Success { success_levels: 4 }
        );
    }

    #[test]
    fn test_critical_success() {
        assert_eq!(resolve_feat(35, false), FeatResult::CriticalSuccess);
        assert_eq!(resolve_feat(50, false), FeatResult::CriticalSuccess);
        assert_eq!(resolve_feat(100, false), FeatResult::CriticalSuccess);
    }

    #[test]
    fn test_success_levels_method() {
        assert_eq!(FeatResult::CriticalFailure.success_levels(), 0);
        assert_eq!(FeatResult::Failure.success_levels(), 0);
        assert_eq!(FeatResult::PartialSuccess.success_levels(), 0);
        assert_eq!(
            FeatResult::Success { success_levels: 3 }.success_levels(),
            3
        );
        assert_eq!(FeatResult::CriticalSuccess.success_levels(), 5);
    }

    #[test]
    fn test_is_success() {
        assert!(!FeatResult::CriticalFailure.is_success());
        assert!(!FeatResult::Failure.is_success());
        assert!(!FeatResult::PartialSuccess.is_success());
        assert!(FeatResult::Success { success_levels: 1 }.is_success());
        assert!(FeatResult::CriticalSuccess.is_success());
    }
}
