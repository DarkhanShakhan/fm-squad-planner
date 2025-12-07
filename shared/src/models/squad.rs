use serde::{Deserialize, Serialize};
use super::player::{Player, Position};

/// Player assigned to a position in the squad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadPlayer {
    pub player_id: i64,
    pub position: Position,   // Position on the pitch
    pub role_id: Option<i64>, // Assigned role
    pub suitability: Option<f32>, // Role suitability percentage
}

/// Squad configuration with starting XI and substitutes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Squad {
    pub id: Option<i64>,
    pub name: String,
    pub formation_id: i64,

    /// Starting XI players with their positions and roles
    pub starting_xi: Vec<SquadPlayer>,

    /// Substitute players
    pub substitutes: Vec<i64>, // Player IDs

    /// Squad statistics
    pub average_rating: Option<f32>,
    pub total_value: Option<i64>,
    pub total_wage: Option<i64>,
}

impl Squad {
    /// Create a new squad
    pub fn new(name: String, formation_id: i64) -> Self {
        Self {
            id: None,
            name,
            formation_id,
            starting_xi: Vec::new(),
            substitutes: Vec::new(),
            average_rating: None,
            total_value: None,
            total_wage: None,
        }
    }

    /// Add a player to starting XI
    pub fn add_starter(&mut self, player: SquadPlayer) {
        self.starting_xi.push(player);
    }

    /// Add a substitute
    pub fn add_substitute(&mut self, player_id: i64) {
        self.substitutes.push(player_id);
    }

    /// Calculate average suitability rating
    pub fn calculate_average_rating(&mut self) {
        let total: f32 = self
            .starting_xi
            .iter()
            .filter_map(|p| p.suitability)
            .sum();

        let count = self
            .starting_xi
            .iter()
            .filter(|p| p.suitability.is_some())
            .count();

        self.average_rating = if count > 0 {
            Some(total / count as f32)
        } else {
            None
        };
    }
}

/// Squad with full player details (for API responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadWithPlayers {
    pub id: Option<i64>,
    pub name: String,
    pub formation_id: i64,
    pub formation_name: String,

    /// Starting XI with full player data
    pub starting_xi: Vec<SquadPlayerWithData>,

    /// Substitute players with full data
    pub substitutes: Vec<Player>,

    /// Squad statistics
    pub average_rating: Option<f32>,
    pub total_value: Option<i64>,
    pub total_wage: Option<i64>,
}

/// Squad player with full player data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadPlayerWithData {
    pub player: Player,
    pub position: Position,
    pub role_id: Option<i64>,
    pub role_name: Option<String>,
    pub suitability: Option<f32>,
}
