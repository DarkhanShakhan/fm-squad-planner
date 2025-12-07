use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::player::Position;

/// Player duty for a role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Duty {
    Defend,
    Support,
    Attack,
    Automatic, // For goalkeepers
}

impl Duty {
    pub fn as_str(&self) -> &str {
        match self {
            Duty::Defend => "Defend",
            Duty::Support => "Support",
            Duty::Attack => "Attack",
            Duty::Automatic => "Automatic",
        }
    }
}

/// Role definition with attribute importance weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Option<i64>,
    pub name: String,
    pub position: Position,  // Position enum
    pub duty: Duty,
    pub description: Option<String>,

    /// Attribute weights (0.0 to 1.0) indicating importance for this role
    /// Key is attribute name, value is weight
    pub attribute_weights: HashMap<String, f32>,

    pub is_custom: bool,
}

impl Role {
    /// Create a new role
    pub fn new(
        name: String,
        position: Position,
        duty: Duty,
        attribute_weights: HashMap<String, f32>,
    ) -> Self {
        Self {
            id: None,
            name,
            position,
            duty,
            description: None,
            attribute_weights,
            is_custom: false,
        }
    }

    /// Create a custom role
    pub fn new_custom(
        name: String,
        position: Position,
        duty: Duty,
        attribute_weights: HashMap<String, f32>,
    ) -> Self {
        Self {
            id: None,
            name,
            position,
            duty,
            description: None,
            attribute_weights,
            is_custom: true,
        }
    }

    /// Get weight for a specific attribute
    pub fn get_weight(&self, attribute: &str) -> f32 {
        *self.attribute_weights.get(attribute).unwrap_or(&0.0)
    }
}

/// Result of role suitability calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleSuitability {
    pub role_id: i64,
    pub role_name: String,
    pub score: f32,        // Raw score
    pub percentage: f32,   // Percentage suitability (0-100)
}

impl RoleSuitability {
    pub fn new(role_id: i64, role_name: String, score: f32, max_score: f32) -> Self {
        let percentage = if max_score > 0.0 {
            (score / max_score) * 100.0
        } else {
            0.0
        };

        Self {
            role_id,
            role_name,
            score,
            percentage: percentage.min(100.0),
        }
    }
}
