use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Team mentality setting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mentality {
    VeryDefensive,
    Defensive,
    Cautious,
    Balanced,
    Positive,
    Attacking,
    VeryAttacking,
}

impl Mentality {
    pub fn as_str(&self) -> &str {
        match self {
            Mentality::VeryDefensive => "Very Defensive",
            Mentality::Defensive => "Defensive",
            Mentality::Cautious => "Cautious",
            Mentality::Balanced => "Balanced",
            Mentality::Positive => "Positive",
            Mentality::Attacking => "Attacking",
            Mentality::VeryAttacking => "Very Attacking",
        }
    }
}

/// Team width setting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Width {
    VeryNarrow,
    Narrow,
    Standard,
    Wide,
    VeryWide,
}

/// Team tempo setting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Tempo {
    VerySlow,
    Slow,
    Standard,
    Fast,
    VeryFast,
}

/// Pressing intensity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PressingIntensity {
    MuchLess,
    Less,
    Standard,
    More,
    MuchMore,
}

/// Defensive line height
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DefensiveLine {
    MuchDeeper,
    Deeper,
    Standard,
    Higher,
    MuchHigher,
}

/// Tactical configuration for a team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactics {
    pub id: Option<i64>,
    pub name: String,

    // Core tactical settings
    pub mentality: Mentality,
    pub width: Width,
    pub tempo: Tempo,
    pub pressing_intensity: PressingIntensity,
    pub defensive_line: DefensiveLine,

    // Additional team instructions (optional)
    pub team_instructions: HashMap<String, bool>,

    // Associated squad
    pub squad_id: Option<i64>,
}

impl Tactics {
    /// Create new tactics with default settings
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            mentality: Mentality::Balanced,
            width: Width::Standard,
            tempo: Tempo::Standard,
            pressing_intensity: PressingIntensity::Standard,
            defensive_line: DefensiveLine::Standard,
            team_instructions: HashMap::new(),
            squad_id: None,
        }
    }

    /// Create tactics for a specific squad
    pub fn new_for_squad(name: String, squad_id: i64) -> Self {
        let mut tactics = Self::new(name);
        tactics.squad_id = Some(squad_id);
        tactics
    }
}
