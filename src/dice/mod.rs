//! Dice module for Fantasy Express RPG
//!
//! Provides core dice rolling mechanics and FEAT table resolution

pub mod feat;
pub mod roll;

pub use feat::resolve_feat;
pub use roll::{roll_2d10_closed, roll_2d10_open, roll_with_modifier};
