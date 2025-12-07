use sqlx::SqlitePool;
use shared::models::Player;
use crate::error::{AppError, AppResult};

pub struct PlayerRepository {
    pool: SqlitePool,
}

impl PlayerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, player: &Player) -> AppResult<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO players (
                name, age, nationality, position,
                corners, crossing, dribbling, finishing, first_touch, free_kick_taking,
                heading, long_shots, long_throws, marking, passing, penalty_taking,
                tackling, technique,
                aggression, anticipation, bravery, composure, concentration, decisions,
                determination, flair, leadership, off_the_ball, positioning, teamwork,
                vision, work_rate,
                acceleration, agility, balance, jumping_reach, natural_fitness, pace,
                stamina, strength,
                aerial_reach, command_of_area, communication, eccentricity, handling,
                kicking, one_on_ones, reflexes, rushing_out, punching, throwing,
                value, wage, contract_expiry
            )
            VALUES (
                ?1, ?2, ?3, ?4,
                ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16,
                ?17, ?18,
                ?19, ?20, ?21, ?22, ?23, ?24,
                ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32,
                ?33, ?34, ?35, ?36, ?37, ?38,
                ?39, ?40,
                ?41, ?42, ?43, ?44, ?45,
                ?46, ?47, ?48, ?49, ?50, ?51,
                ?52, ?53, ?54
            )
            "#,
            player.name, player.age, player.nationality, player.position.as_str(),
            player.corners, player.crossing, player.dribbling, player.finishing,
            player.first_touch, player.free_kick_taking, player.heading, player.long_shots,
            player.long_throws, player.marking, player.passing, player.penalty_taking,
            player.tackling, player.technique,
            player.aggression, player.anticipation, player.bravery, player.composure,
            player.concentration, player.decisions, player.determination, player.flair,
            player.leadership, player.off_the_ball, player.positioning, player.teamwork,
            player.vision, player.work_rate,
            player.acceleration, player.agility, player.balance, player.jumping_reach,
            player.natural_fitness, player.pace, player.stamina, player.strength,
            player.aerial_reach, player.command_of_area, player.communication,
            player.eccentricity, player.handling, player.kicking, player.one_on_ones,
            player.reflexes, player.rushing_out, player.punching, player.throwing,
            player.value, player.wage, player.contract_expiry
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Player> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM players WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let position = r.position.parse().map_err(|_| {
                    AppError::InternalError(format!("Invalid position: {}", r.position))
                })?;

                Ok(Player {
                    id: Some(r.id),
                    name: r.name,
                    age: r.age,
                    nationality: r.nationality,
                    position,
                    corners: r.corners,
                    crossing: r.crossing,
                    dribbling: r.dribbling,
                    finishing: r.finishing,
                    first_touch: r.first_touch,
                    free_kick_taking: r.free_kick_taking,
                    heading: r.heading,
                    long_shots: r.long_shots,
                    long_throws: r.long_throws,
                    marking: r.marking,
                    passing: r.passing,
                    penalty_taking: r.penalty_taking,
                    tackling: r.tackling,
                    technique: r.technique,
                    aggression: r.aggression,
                    anticipation: r.anticipation,
                    bravery: r.bravery,
                    composure: r.composure,
                    concentration: r.concentration,
                    decisions: r.decisions,
                    determination: r.determination,
                    flair: r.flair,
                    leadership: r.leadership,
                    off_the_ball: r.off_the_ball,
                    positioning: r.positioning,
                    teamwork: r.teamwork,
                    vision: r.vision,
                    work_rate: r.work_rate,
                    acceleration: r.acceleration,
                    agility: r.agility,
                    balance: r.balance,
                    jumping_reach: r.jumping_reach,
                    natural_fitness: r.natural_fitness,
                    pace: r.pace,
                    stamina: r.stamina,
                    strength: r.strength,
                    aerial_reach: r.aerial_reach,
                    command_of_area: r.command_of_area,
                    communication: r.communication,
                    eccentricity: r.eccentricity,
                    handling: r.handling,
                    kicking: r.kicking,
                    one_on_ones: r.one_on_ones,
                    reflexes: r.reflexes,
                    rushing_out: r.rushing_out,
                    punching: r.punching,
                    throwing: r.throwing,
                    value: r.value,
                    wage: r.wage,
                    contract_expiry: r.contract_expiry,
                })
            }
            None => Err(AppError::NotFound(format!("Player with id {} not found", id))),
        }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Player>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM players ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut players = Vec::new();
        for r in rows {
            let position = r.position.parse().map_err(|_| {
                AppError::InternalError(format!("Invalid position: {}", r.position))
            })?;

            players.push(Player {
                id: Some(r.id),
                name: r.name,
                age: r.age,
                nationality: r.nationality,
                position,
                corners: r.corners,
                crossing: r.crossing,
                dribbling: r.dribbling,
                finishing: r.finishing,
                first_touch: r.first_touch,
                free_kick_taking: r.free_kick_taking,
                heading: r.heading,
                long_shots: r.long_shots,
                long_throws: r.long_throws,
                marking: r.marking,
                passing: r.passing,
                penalty_taking: r.penalty_taking,
                tackling: r.tackling,
                technique: r.technique,
                aggression: r.aggression,
                anticipation: r.anticipation,
                bravery: r.bravery,
                composure: r.composure,
                concentration: r.concentration,
                decisions: r.decisions,
                determination: r.determination,
                flair: r.flair,
                leadership: r.leadership,
                off_the_ball: r.off_the_ball,
                positioning: r.positioning,
                teamwork: r.teamwork,
                vision: r.vision,
                work_rate: r.work_rate,
                acceleration: r.acceleration,
                agility: r.agility,
                balance: r.balance,
                jumping_reach: r.jumping_reach,
                natural_fitness: r.natural_fitness,
                pace: r.pace,
                stamina: r.stamina,
                strength: r.strength,
                aerial_reach: r.aerial_reach,
                command_of_area: r.command_of_area,
                communication: r.communication,
                eccentricity: r.eccentricity,
                handling: r.handling,
                kicking: r.kicking,
                one_on_ones: r.one_on_ones,
                reflexes: r.reflexes,
                rushing_out: r.rushing_out,
                punching: r.punching,
                throwing: r.throwing,
                value: r.value,
                wage: r.wage,
                contract_expiry: r.contract_expiry,
            });
        }

        Ok(players)
    }

    pub async fn update(&self, id: i64, player: &Player) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE players SET
                name = ?1, age = ?2, nationality = ?3, position = ?4,
                corners = ?5, crossing = ?6, dribbling = ?7, finishing = ?8,
                first_touch = ?9, free_kick_taking = ?10, heading = ?11, long_shots = ?12,
                long_throws = ?13, marking = ?14, passing = ?15, penalty_taking = ?16,
                tackling = ?17, technique = ?18,
                aggression = ?19, anticipation = ?20, bravery = ?21, composure = ?22,
                concentration = ?23, decisions = ?24, determination = ?25, flair = ?26,
                leadership = ?27, off_the_ball = ?28, positioning = ?29, teamwork = ?30,
                vision = ?31, work_rate = ?32,
                acceleration = ?33, agility = ?34, balance = ?35, jumping_reach = ?36,
                natural_fitness = ?37, pace = ?38, stamina = ?39, strength = ?40,
                aerial_reach = ?41, command_of_area = ?42, communication = ?43,
                eccentricity = ?44, handling = ?45, kicking = ?46, one_on_ones = ?47,
                reflexes = ?48, rushing_out = ?49, punching = ?50, throwing = ?51,
                value = ?52, wage = ?53, contract_expiry = ?54
            WHERE id = ?55
            "#,
            player.name, player.age, player.nationality, player.position.as_str(),
            player.corners, player.crossing, player.dribbling, player.finishing,
            player.first_touch, player.free_kick_taking, player.heading, player.long_shots,
            player.long_throws, player.marking, player.passing, player.penalty_taking,
            player.tackling, player.technique,
            player.aggression, player.anticipation, player.bravery, player.composure,
            player.concentration, player.decisions, player.determination, player.flair,
            player.leadership, player.off_the_ball, player.positioning, player.teamwork,
            player.vision, player.work_rate,
            player.acceleration, player.agility, player.balance, player.jumping_reach,
            player.natural_fitness, player.pace, player.stamina, player.strength,
            player.aerial_reach, player.command_of_area, player.communication,
            player.eccentricity, player.handling, player.kicking, player.one_on_ones,
            player.reflexes, player.rushing_out, player.punching, player.throwing,
            player.value, player.wage, player.contract_expiry,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Player with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM players WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Player with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn search_by_position(&self, position: &str) -> AppResult<Vec<Player>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM players WHERE position = ?1 ORDER BY name
            "#,
            position
        )
        .fetch_all(&self.pool)
        .await?;

        let mut players = Vec::new();
        for r in rows {
            let position = r.position.parse().map_err(|_| {
                AppError::InternalError(format!("Invalid position: {}", r.position))
            })?;

            players.push(Player {
                id: Some(r.id),
                name: r.name,
                age: r.age,
                nationality: r.nationality,
                position,
                corners: r.corners,
                crossing: r.crossing,
                dribbling: r.dribbling,
                finishing: r.finishing,
                first_touch: r.first_touch,
                free_kick_taking: r.free_kick_taking,
                heading: r.heading,
                long_shots: r.long_shots,
                long_throws: r.long_throws,
                marking: r.marking,
                passing: r.passing,
                penalty_taking: r.penalty_taking,
                tackling: r.tackling,
                technique: r.technique,
                aggression: r.aggression,
                anticipation: r.anticipation,
                bravery: r.bravery,
                composure: r.composure,
                concentration: r.concentration,
                decisions: r.decisions,
                determination: r.determination,
                flair: r.flair,
                leadership: r.leadership,
                off_the_ball: r.off_the_ball,
                positioning: r.positioning,
                teamwork: r.teamwork,
                vision: r.vision,
                work_rate: r.work_rate,
                acceleration: r.acceleration,
                agility: r.agility,
                balance: r.balance,
                jumping_reach: r.jumping_reach,
                natural_fitness: r.natural_fitness,
                pace: r.pace,
                stamina: r.stamina,
                strength: r.strength,
                aerial_reach: r.aerial_reach,
                command_of_area: r.command_of_area,
                communication: r.communication,
                eccentricity: r.eccentricity,
                handling: r.handling,
                kicking: r.kicking,
                one_on_ones: r.one_on_ones,
                reflexes: r.reflexes,
                rushing_out: r.rushing_out,
                punching: r.punching,
                throwing: r.throwing,
                value: r.value,
                wage: r.wage,
                contract_expiry: r.contract_expiry,
            });
        }

        Ok(players)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::Position;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_and_get_player() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let mut player = Player::new("Test Player".to_string(), 25, Position::STC);
        player.finishing = Some(15);
        player.pace = Some(18);

        let id = repo.create(&player).await.unwrap();
        assert!(id > 0);

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Test Player");
        assert_eq!(retrieved.age, 25);
        assert_eq!(retrieved.position, Position::STC);
        assert_eq!(retrieved.finishing, Some(15));
        assert_eq!(retrieved.pace, Some(18));
    }

    #[tokio::test]
    async fn test_get_all_players() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let player1 = Player::new("Player A".to_string(), 25, Position::STC);
        let player2 = Player::new("Player B".to_string(), 28, Position::MC);

        repo.create(&player1).await.unwrap();
        repo.create(&player2).await.unwrap();

        let players = repo.get_all().await.unwrap();
        assert_eq!(players.len(), 2);
    }

    #[tokio::test]
    async fn test_update_player() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let player = Player::new("Test Player".to_string(), 25, Position::STC);
        let id = repo.create(&player).await.unwrap();

        let mut updated_player = Player::new("Updated Player".to_string(), 26, Position::AMC);
        updated_player.finishing = Some(20);

        repo.update(id, &updated_player).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Updated Player");
        assert_eq!(retrieved.age, 26);
        assert_eq!(retrieved.position, Position::AMC);
        assert_eq!(retrieved.finishing, Some(20));
    }

    #[tokio::test]
    async fn test_delete_player() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let player = Player::new("Test Player".to_string(), 25, Position::STC);
        let id = repo.create(&player).await.unwrap();

        repo.delete(id).await.unwrap();

        let result = repo.get_by_id(id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_by_position() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let player1 = Player::new("Striker 1".to_string(), 25, Position::STC);
        let player2 = Player::new("Striker 2".to_string(), 28, Position::STC);
        let player3 = Player::new("Midfielder".to_string(), 26, Position::MC);

        repo.create(&player1).await.unwrap();
        repo.create(&player2).await.unwrap();
        repo.create(&player3).await.unwrap();

        let strikers = repo.search_by_position("STC").await.unwrap();
        assert_eq!(strikers.len(), 2);
        assert!(strikers.iter().all(|p| p.position == Position::STC));
    }

    #[tokio::test]
    async fn test_get_nonexistent_player() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let result = repo.get_by_id(999).await;
        assert!(result.is_err());
        match result {
            Err(AppError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_update_nonexistent_player() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let player = Player::new("Test".to_string(), 25, Position::STC);
        let result = repo.update(999, &player).await;
        assert!(result.is_err());
        match result {
            Err(AppError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_delete_nonexistent_player() {
        let pool = setup_test_db().await;
        let repo = PlayerRepository::new(pool);

        let result = repo.delete(999).await;
        assert!(result.is_err());
        match result {
            Err(AppError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }
}
