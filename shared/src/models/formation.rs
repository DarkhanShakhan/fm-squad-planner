use serde::{Deserialize, Serialize};
use super::player::Position;
use super::tactical_role::TacticalRole;

/// Position on the pitch with coordinates and optional assigned tactical role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationPosition {
    pub position: Position,  // Position enum
    pub x: f32,             // X coordinate (0-100, left to right)
    pub y: f32,             // Y coordinate (0-100, bottom to top)
    pub tactical_role: Option<TacticalRole>, // Optional assigned tactical role
}

/// Formation configuration with position layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formation {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub positions: Vec<FormationPosition>,
    pub is_custom: bool,
}

impl Formation {
    /// Create a new formation
    pub fn new(name: String, positions: Vec<FormationPosition>) -> Self {
        Self {
            id: None,
            name,
            description: None,
            positions,
            is_custom: false,
        }
    }

    /// Create a custom formation
    pub fn new_custom(name: String, positions: Vec<FormationPosition>) -> Self {
        Self {
            id: None,
            name,
            description: None,
            positions,
            is_custom: true,
        }
    }

    /// Get number of players in formation
    pub fn player_count(&self) -> usize {
        self.positions.len()
    }

    /// Validate formation has 11 positions (including GK)
    pub fn is_valid(&self) -> bool {
        self.player_count() == 11
    }
}
