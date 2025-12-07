use serde::{Deserialize, Serialize};
use super::player::Position;

/// Player duty for a role
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Duty {
    Defend,
    Support,
    Attack,
    Automatic, // For goalkeepers only
}

/// All available roles in Football Manager
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RoleType {
    // Goalkeeper roles
    Goalkeeper,
    SweeperKeeper,

    // Defender roles
    FullBack,
    WingBack,
    CompleteWingBack,
    InvertedWingBack,
    CentralDefender,
    BallPlayingDefender,
    Libero,
    NoChallengeDefender,

    // Defensive Midfielder roles
    Anchor,
    BallWinningMidfielder,
    DeepLyingPlaymaker,
    HalfBack,
    Regista,
    SegundoVolante,

    // Midfielder roles
    BoxToBoxMidfielder,
    CentralMidfielder,
    Carrilero,
    DeepLyingMidfielderSupport,
    Mezzala,
    RoamingPlaymaker,

    // Wide Midfielder roles
    Winger,
    InvertedWinger,
    WideMidfielder,
    WidePlaymaker,
    DefensiveWinger,

    // Attacking Midfielder roles
    AdvancedPlaymaker,
    AttackingMidfielder,
    Enganche,
    ShadowStriker,
    Trequartista,

    // Forward roles
    AdvancedForward,
    CompleteForward,
    DeepLyingForward,
    FalseNine,
    InsideForward,
    Poacher,
    PressingForward,
    Raumdeuter,
    TargetMan,
    TrequartistaForward,
}

/// Valid tactical role combining position, role type, and duty
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TacticalRole {
    pub position: Position,
    pub role: RoleType,
    pub duty: Duty,
}

impl TacticalRole {
    /// Create a new tactical role with validation
    pub fn new(position: Position, role: RoleType, duty: Duty) -> Result<Self, String> {
        if Self::is_valid_combination(position, role, duty) {
            Ok(Self {
                position,
                role,
                duty,
            })
        } else {
            Err(format!(
                "Invalid combination: {:?} cannot play {:?} on {:?} duty",
                role, position, duty
            ))
        }
    }

    /// Check if a combination of position, role, and duty is valid
    pub fn is_valid_combination(position: Position, role: RoleType, duty: Duty) -> bool {
        use Position::*;
        use RoleType::*;
        use Duty::*;

        match (position, role, duty) {
            // Goalkeeper roles - only automatic duty
            (GK, Goalkeeper, Automatic) => true,
            (GK, SweeperKeeper, Automatic) => true,
            (GK, _, _) => false,

            // Defender roles (DR, DC, DL)
            (DR | DL, FullBack, Defend | Support | Attack) => true,
            (DR | DL, WingBack, Defend | Support | Attack) => true,
            (DR | DL, CompleteWingBack, Support | Attack) => true,
            (DR | DL, InvertedWingBack, Defend | Support | Attack) => true,
            (DR | DL, NoChallengeDefender, Defend) => true,

            (DC, CentralDefender, Defend | Support) => true,
            (DC, BallPlayingDefender, Defend | Support) => true,
            (DC, Libero, Defend | Support) => true,
            (DC, NoChallengeDefender, Defend) => true,

            // Wing-back positions (WBR, WBL)
            (WBR | WBL, WingBack, Defend | Support | Attack) => true,
            (WBR | WBL, CompleteWingBack, Support | Attack) => true,
            (WBR | WBL, InvertedWingBack, Defend | Support | Attack) => true,

            // Defensive Midfielder (DMC)
            (DMC, Anchor, Defend) => true,
            (DMC, BallWinningMidfielder, Defend | Support) => true,
            (DMC, DeepLyingPlaymaker, Defend | Support) => true,
            (DMC, HalfBack, Defend) => true,
            (DMC, Regista, Support) => true,
            (DMC, SegundoVolante, Support | Attack) => true,

            // Central Midfielder (MC)
            (MC, BoxToBoxMidfielder, Support) => true,
            (MC, CentralMidfielder, Defend | Support | Attack) => true,
            (MC, Carrilero, Support) => true,
            (MC, DeepLyingMidfielderSupport, Support) => true,
            (MC, Mezzala, Support | Attack) => true,
            (MC, RoamingPlaymaker, Support) => true,
            (MC, BallWinningMidfielder, Defend | Support) => true,
            (MC, AdvancedPlaymaker, Support | Attack) => true,

            // Wide Midfielders (MR, ML)
            (MR | ML, Winger, Support | Attack) => true,
            (MR | ML, InvertedWinger, Support | Attack) => true,
            (MR | ML, WideMidfielder, Defend | Support | Attack) => true,
            (MR | ML, WidePlaymaker, Support | Attack) => true,
            (MR | ML, DefensiveWinger, Defend | Support) => true,

            // Attacking Midfielders (AMR, AML, AMC)
            (AMR | AML, Winger, Support | Attack) => true,
            (AMR | AML, InvertedWinger, Support | Attack) => true,
            (AMR | AML, WidePlaymaker, Support | Attack) => true,
            (AMR | AML, InsideForward, Support | Attack) => true,
            (AMR | AML, Raumdeuter, Attack) => true,

            (AMC, AdvancedPlaymaker, Support | Attack) => true,
            (AMC, AttackingMidfielder, Support | Attack) => true,
            (AMC, Enganche, Support) => true,
            (AMC, ShadowStriker, Attack) => true,
            (AMC, Trequartista, Support) => true,

            // Strikers (STC)
            (STC, AdvancedForward, Attack) => true,
            (STC, CompleteForward, Support | Attack) => true,
            (STC, DeepLyingForward, Support | Attack) => true,
            (STC, FalseNine, Support) => true,
            (STC, Poacher, Attack) => true,
            (STC, PressingForward, Defend | Support | Attack) => true,
            (STC, TargetMan, Support | Attack) => true,
            (STC, TrequartistaForward, Support) => true,

            // All other combinations are invalid
            _ => false,
        }
    }

    /// Get all valid tactical roles for a position
    pub fn get_all_for_position(position: Position) -> Vec<TacticalRole> {
        let mut roles = Vec::new();

        // Try all role types with all duties
        for role in Self::all_role_types() {
            for duty in Self::all_duties() {
                if Self::is_valid_combination(position, role, duty) {
                    roles.push(TacticalRole {
                        position,
                        role,
                        duty,
                    });
                }
            }
        }

        roles
    }

    /// Get all role types
    fn all_role_types() -> Vec<RoleType> {
        use RoleType::*;
        vec![
            // GK
            Goalkeeper,
            SweeperKeeper,
            // Defenders
            FullBack,
            WingBack,
            CompleteWingBack,
            InvertedWingBack,
            CentralDefender,
            BallPlayingDefender,
            Libero,
            NoChallengeDefender,
            // DM
            Anchor,
            BallWinningMidfielder,
            DeepLyingPlaymaker,
            HalfBack,
            Regista,
            SegundoVolante,
            // CM
            BoxToBoxMidfielder,
            CentralMidfielder,
            Carrilero,
            DeepLyingMidfielderSupport,
            Mezzala,
            RoamingPlaymaker,
            // Wide
            Winger,
            InvertedWinger,
            WideMidfielder,
            WidePlaymaker,
            DefensiveWinger,
            // AM
            AdvancedPlaymaker,
            AttackingMidfielder,
            Enganche,
            ShadowStriker,
            Trequartista,
            // Forward
            AdvancedForward,
            CompleteForward,
            DeepLyingForward,
            FalseNine,
            InsideForward,
            Poacher,
            PressingForward,
            Raumdeuter,
            TargetMan,
            TrequartistaForward,
        ]
    }

    /// Get all duties
    fn all_duties() -> Vec<Duty> {
        vec![Duty::Defend, Duty::Support, Duty::Attack, Duty::Automatic]
    }

    /// Get role name as string
    pub fn role_name(&self) -> &'static str {
        use RoleType::*;
        match self.role {
            Goalkeeper => "Goalkeeper",
            SweeperKeeper => "Sweeper Keeper",
            FullBack => "Full Back",
            WingBack => "Wing Back",
            CompleteWingBack => "Complete Wing Back",
            InvertedWingBack => "Inverted Wing Back",
            CentralDefender => "Central Defender",
            BallPlayingDefender => "Ball-Playing Defender",
            Libero => "Libero",
            NoChallengeDefender => "No-Challenge Defender",
            Anchor => "Anchor",
            BallWinningMidfielder => "Ball-Winning Midfielder",
            DeepLyingPlaymaker => "Deep-Lying Playmaker",
            HalfBack => "Half Back",
            Regista => "Regista",
            SegundoVolante => "Segundo Volante",
            BoxToBoxMidfielder => "Box-to-Box Midfielder",
            CentralMidfielder => "Central Midfielder",
            Carrilero => "Carrilero",
            DeepLyingMidfielderSupport => "Deep-Lying Midfielder",
            Mezzala => "Mezzala",
            RoamingPlaymaker => "Roaming Playmaker",
            Winger => "Winger",
            InvertedWinger => "Inverted Winger",
            WideMidfielder => "Wide Midfielder",
            WidePlaymaker => "Wide Playmaker",
            DefensiveWinger => "Defensive Winger",
            AdvancedPlaymaker => "Advanced Playmaker",
            AttackingMidfielder => "Attacking Midfielder",
            Enganche => "Enganche",
            ShadowStriker => "Shadow Striker",
            Trequartista => "Trequartista",
            AdvancedForward => "Advanced Forward",
            CompleteForward => "Complete Forward",
            DeepLyingForward => "Deep-Lying Forward",
            FalseNine => "False Nine",
            InsideForward => "Inside Forward",
            Poacher => "Poacher",
            PressingForward => "Pressing Forward",
            Raumdeuter => "Raumdeuter",
            TargetMan => "Target Man",
            TrequartistaForward => "Trequartista Forward",
        }
    }

    /// Get duty name as string
    pub fn duty_name(&self) -> &'static str {
        match self.duty {
            Duty::Defend => "Defend",
            Duty::Support => "Support",
            Duty::Attack => "Attack",
            Duty::Automatic => "Automatic",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_goalkeeper() {
        assert!(TacticalRole::new(Position::GK, RoleType::Goalkeeper, Duty::Automatic).is_ok());
        assert!(TacticalRole::new(Position::GK, RoleType::Goalkeeper, Duty::Defend).is_err());
    }

    #[test]
    fn test_valid_fullback() {
        assert!(TacticalRole::new(Position::DR, RoleType::FullBack, Duty::Support).is_ok());
        assert!(TacticalRole::new(Position::DR, RoleType::FullBack, Duty::Attack).is_ok());
        assert!(TacticalRole::new(Position::DR, RoleType::FullBack, Duty::Automatic).is_err());
    }

    #[test]
    fn test_invalid_position_role() {
        // Can't have a striker role at defender position
        assert!(TacticalRole::new(Position::DC, RoleType::Poacher, Duty::Attack).is_err());
    }

    #[test]
    fn test_get_all_for_position() {
        let gk_roles = TacticalRole::get_all_for_position(Position::GK);
        assert_eq!(gk_roles.len(), 2); // Goalkeeper and Sweeper Keeper

        let striker_roles = TacticalRole::get_all_for_position(Position::STC);
        assert!(striker_roles.len() > 5); // Multiple striker roles
    }
}
