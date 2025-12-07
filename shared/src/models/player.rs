use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Football Manager player position
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Position {
    GK,   // Goalkeeper
    DR,   // Defender Right
    DC,   // Defender Center
    DL,   // Defender Left
    WBR,  // Wing Back Right
    WBL,  // Wing Back Left
    DMC,  // Defensive Midfielder Center
    MC,   // Midfielder Center
    MR,   // Midfielder Right
    ML,   // Midfielder Left
    AMR,  // Attacking Midfielder Right
    AML,  // Attacking Midfielder Left
    AMC,  // Attacking Midfielder Center
    STC,  // Striker Center
}

impl Position {
    pub fn as_str(&self) -> &str {
        match self {
            Position::GK => "GK",
            Position::DR => "DR",
            Position::DC => "DC",
            Position::DL => "DL",
            Position::WBR => "WBR",
            Position::WBL => "WBL",
            Position::DMC => "DMC",
            Position::MC => "MC",
            Position::MR => "MR",
            Position::ML => "ML",
            Position::AMR => "AMR",
            Position::AML => "AML",
            Position::AMC => "AMC",
            Position::STC => "STC",
        }
    }
}

/// Complete player model with all Football Manager attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Option<i64>,
    pub name: String,
    pub age: i32,
    pub nationality: Option<String>,
    pub position: Position,

    // Technical attributes (1-20)
    pub corners: Option<i32>,
    pub crossing: Option<i32>,
    pub dribbling: Option<i32>,
    pub finishing: Option<i32>,
    pub first_touch: Option<i32>,
    pub free_kick_taking: Option<i32>,
    pub heading: Option<i32>,
    pub long_shots: Option<i32>,
    pub long_throws: Option<i32>,
    pub marking: Option<i32>,
    pub passing: Option<i32>,
    pub penalty_taking: Option<i32>,
    pub tackling: Option<i32>,
    pub technique: Option<i32>,

    // Mental attributes (1-20)
    pub aggression: Option<i32>,
    pub anticipation: Option<i32>,
    pub bravery: Option<i32>,
    pub composure: Option<i32>,
    pub concentration: Option<i32>,
    pub decisions: Option<i32>,
    pub determination: Option<i32>,
    pub flair: Option<i32>,
    pub leadership: Option<i32>,
    pub off_the_ball: Option<i32>,
    pub positioning: Option<i32>,
    pub teamwork: Option<i32>,
    pub vision: Option<i32>,
    pub work_rate: Option<i32>,

    // Physical attributes (1-20)
    pub acceleration: Option<i32>,
    pub agility: Option<i32>,
    pub balance: Option<i32>,
    pub jumping_reach: Option<i32>,
    pub natural_fitness: Option<i32>,
    pub pace: Option<i32>,
    pub stamina: Option<i32>,
    pub strength: Option<i32>,

    // Goalkeeper attributes (1-20, only for GKs)
    pub aerial_reach: Option<i32>,
    pub command_of_area: Option<i32>,
    pub communication: Option<i32>,
    pub eccentricity: Option<i32>,
    pub handling: Option<i32>,
    pub kicking: Option<i32>,
    pub one_on_ones: Option<i32>,
    pub reflexes: Option<i32>,
    pub rushing_out: Option<i32>,
    pub punching: Option<i32>,
    pub throwing: Option<i32>,

    // Additional info
    pub value: Option<i64>,
    pub wage: Option<i64>,
    pub contract_expiry: Option<NaiveDate>,
}

impl Player {
    /// Create a new player with required fields
    pub fn new(name: String, age: i32, position: Position) -> Self {
        Self {
            id: None,
            name,
            age,
            nationality: None,
            position,
            corners: None,
            crossing: None,
            dribbling: None,
            finishing: None,
            first_touch: None,
            free_kick_taking: None,
            heading: None,
            long_shots: None,
            long_throws: None,
            marking: None,
            passing: None,
            penalty_taking: None,
            tackling: None,
            technique: None,
            aggression: None,
            anticipation: None,
            bravery: None,
            composure: None,
            concentration: None,
            decisions: None,
            determination: None,
            flair: None,
            leadership: None,
            off_the_ball: None,
            positioning: None,
            teamwork: None,
            vision: None,
            work_rate: None,
            acceleration: None,
            agility: None,
            balance: None,
            jumping_reach: None,
            natural_fitness: None,
            pace: None,
            stamina: None,
            strength: None,
            aerial_reach: None,
            command_of_area: None,
            communication: None,
            eccentricity: None,
            handling: None,
            kicking: None,
            one_on_ones: None,
            reflexes: None,
            rushing_out: None,
            punching: None,
            throwing: None,
            value: None,
            wage: None,
            contract_expiry: None,
        }
    }

    /// Check if this is a goalkeeper
    pub fn is_goalkeeper(&self) -> bool {
        matches!(self.position, Position::GK)
    }
}
