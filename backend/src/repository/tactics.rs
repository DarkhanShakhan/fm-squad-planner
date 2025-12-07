use sqlx::SqlitePool;
use shared::models::{Tactics, Mentality, Width, Tempo, PressingIntensity, DefensiveLine};
use crate::error::{AppError, AppResult};
use std::collections::HashMap;

pub struct TacticsRepository {
    pool: SqlitePool,
}

impl TacticsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, tactics: &Tactics) -> AppResult<i64> {
        let team_instructions_json = serde_json::to_string(&tactics.team_instructions)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize team_instructions: {}", e)))?;

        let result = sqlx::query!(
            r#"
            INSERT INTO tactics (
                name, mentality, width, tempo, pressing_intensity, defensive_line,
                team_instructions, squad_id
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            tactics.name,
            tactics.mentality.as_str(),
            serde_json::to_string(&tactics.width).unwrap(),
            serde_json::to_string(&tactics.tempo).unwrap(),
            serde_json::to_string(&tactics.pressing_intensity).unwrap(),
            serde_json::to_string(&tactics.defensive_line).unwrap(),
            team_instructions_json,
            tactics.squad_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Tactics> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM tactics WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let mentality = match r.mentality.as_str() {
                    "Very Defensive" => Mentality::VeryDefensive,
                    "Defensive" => Mentality::Defensive,
                    "Cautious" => Mentality::Cautious,
                    "Balanced" => Mentality::Balanced,
                    "Positive" => Mentality::Positive,
                    "Attacking" => Mentality::Attacking,
                    "Very Attacking" => Mentality::VeryAttacking,
                    _ => return Err(AppError::InternalError(format!("Invalid mentality: {}", r.mentality))),
                };

                let width: Width = serde_json::from_str(&r.width)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse width: {}", e)))?;

                let tempo: Tempo = serde_json::from_str(&r.tempo)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse tempo: {}", e)))?;

                let pressing_intensity: PressingIntensity = serde_json::from_str(&r.pressing_intensity)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse pressing_intensity: {}", e)))?;

                let defensive_line: DefensiveLine = serde_json::from_str(&r.defensive_line)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse defensive_line: {}", e)))?;

                let team_instructions: HashMap<String, bool> = serde_json::from_str(&r.team_instructions)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse team_instructions: {}", e)))?;

                Ok(Tactics {
                    id: Some(r.id),
                    name: r.name,
                    mentality,
                    width,
                    tempo,
                    pressing_intensity,
                    defensive_line,
                    team_instructions,
                    squad_id: r.squad_id,
                })
            }
            None => Err(AppError::NotFound(format!("Tactics with id {} not found", id))),
        }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Tactics>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM tactics ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut tactics_list = Vec::new();
        for r in rows {
            let mentality = match r.mentality.as_str() {
                "Very Defensive" => Mentality::VeryDefensive,
                "Defensive" => Mentality::Defensive,
                "Cautious" => Mentality::Cautious,
                "Balanced" => Mentality::Balanced,
                "Positive" => Mentality::Positive,
                "Attacking" => Mentality::Attacking,
                "Very Attacking" => Mentality::VeryAttacking,
                _ => return Err(AppError::InternalError(format!("Invalid mentality: {}", r.mentality))),
            };

            let width: Width = serde_json::from_str(&r.width)
                .map_err(|e| AppError::InternalError(format!("Failed to parse width: {}", e)))?;

            let tempo: Tempo = serde_json::from_str(&r.tempo)
                .map_err(|e| AppError::InternalError(format!("Failed to parse tempo: {}", e)))?;

            let pressing_intensity: PressingIntensity = serde_json::from_str(&r.pressing_intensity)
                .map_err(|e| AppError::InternalError(format!("Failed to parse pressing_intensity: {}", e)))?;

            let defensive_line: DefensiveLine = serde_json::from_str(&r.defensive_line)
                .map_err(|e| AppError::InternalError(format!("Failed to parse defensive_line: {}", e)))?;

            let team_instructions: HashMap<String, bool> = serde_json::from_str(&r.team_instructions)
                .map_err(|e| AppError::InternalError(format!("Failed to parse team_instructions: {}", e)))?;

            tactics_list.push(Tactics {
                id: Some(r.id),
                name: r.name,
                mentality,
                width,
                tempo,
                pressing_intensity,
                defensive_line,
                team_instructions,
                squad_id: r.squad_id,
            });
        }

        Ok(tactics_list)
    }

    pub async fn update(&self, id: i64, tactics: &Tactics) -> AppResult<()> {
        let team_instructions_json = serde_json::to_string(&tactics.team_instructions)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize team_instructions: {}", e)))?;

        let result = sqlx::query!(
            r#"
            UPDATE tactics SET
                name = ?1,
                mentality = ?2,
                width = ?3,
                tempo = ?4,
                pressing_intensity = ?5,
                defensive_line = ?6,
                team_instructions = ?7,
                squad_id = ?8
            WHERE id = ?9
            "#,
            tactics.name,
            tactics.mentality.as_str(),
            serde_json::to_string(&tactics.width).unwrap(),
            serde_json::to_string(&tactics.tempo).unwrap(),
            serde_json::to_string(&tactics.pressing_intensity).unwrap(),
            serde_json::to_string(&tactics.defensive_line).unwrap(),
            team_instructions_json,
            tactics.squad_id,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Tactics with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tactics WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Tactics with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn get_by_squad(&self, squad_id: i64) -> AppResult<Vec<Tactics>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM tactics WHERE squad_id = ?1 ORDER BY name
            "#,
            squad_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut tactics_list = Vec::new();
        for r in rows {
            let mentality = match r.mentality.as_str() {
                "Very Defensive" => Mentality::VeryDefensive,
                "Defensive" => Mentality::Defensive,
                "Cautious" => Mentality::Cautious,
                "Balanced" => Mentality::Balanced,
                "Positive" => Mentality::Positive,
                "Attacking" => Mentality::Attacking,
                "Very Attacking" => Mentality::VeryAttacking,
                _ => return Err(AppError::InternalError(format!("Invalid mentality: {}", r.mentality))),
            };

            let width: Width = serde_json::from_str(&r.width)
                .map_err(|e| AppError::InternalError(format!("Failed to parse width: {}", e)))?;

            let tempo: Tempo = serde_json::from_str(&r.tempo)
                .map_err(|e| AppError::InternalError(format!("Failed to parse tempo: {}", e)))?;

            let pressing_intensity: PressingIntensity = serde_json::from_str(&r.pressing_intensity)
                .map_err(|e| AppError::InternalError(format!("Failed to parse pressing_intensity: {}", e)))?;

            let defensive_line: DefensiveLine = serde_json::from_str(&r.defensive_line)
                .map_err(|e| AppError::InternalError(format!("Failed to parse defensive_line: {}", e)))?;

            let team_instructions: HashMap<String, bool> = serde_json::from_str(&r.team_instructions)
                .map_err(|e| AppError::InternalError(format!("Failed to parse team_instructions: {}", e)))?;

            tactics_list.push(Tactics {
                id: Some(r.id),
                name: r.name,
                mentality,
                width,
                tempo,
                pressing_intensity,
                defensive_line,
                team_instructions,
                squad_id: r.squad_id,
            });
        }

        Ok(tactics_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    fn create_test_tactics() -> Tactics {
        let mut tactics = Tactics::new("Test Tactics".to_string());
        tactics.mentality = Mentality::Attacking;
        tactics.width = Width::Wide;
        tactics.tempo = Tempo::Fast;

        let mut instructions = HashMap::new();
        instructions.insert("counter_press".to_string(), true);
        instructions.insert("play_out_of_defence".to_string(), true);
        tactics.team_instructions = instructions;

        tactics
    }

    #[tokio::test]
    async fn test_create_and_get_tactics() {
        let pool = setup_test_db().await;
        let repo = TacticsRepository::new(pool);

        let tactics = create_test_tactics();
        let id = repo.create(&tactics).await.unwrap();
        assert!(id > 0);

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Test Tactics");
        assert_eq!(retrieved.mentality, Mentality::Attacking);
        assert_eq!(retrieved.width, Width::Wide);
        assert_eq!(retrieved.tempo, Tempo::Fast);
        assert_eq!(retrieved.team_instructions.get("counter_press"), Some(&true));
    }

    #[tokio::test]
    async fn test_get_all_tactics() {
        let pool = setup_test_db().await;
        let repo = TacticsRepository::new(pool);

        let tactics1 = create_test_tactics();
        let mut tactics2 = create_test_tactics();
        tactics2.name = "Tactics 2".to_string();

        repo.create(&tactics1).await.unwrap();
        repo.create(&tactics2).await.unwrap();

        let tactics_list = repo.get_all().await.unwrap();
        assert_eq!(tactics_list.len(), 2);
    }

    #[tokio::test]
    async fn test_update_tactics() {
        let pool = setup_test_db().await;
        let repo = TacticsRepository::new(pool);

        let tactics = create_test_tactics();
        let id = repo.create(&tactics).await.unwrap();

        let mut updated_tactics = create_test_tactics();
        updated_tactics.name = "Updated Tactics".to_string();
        updated_tactics.mentality = Mentality::Defensive;

        repo.update(id, &updated_tactics).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Updated Tactics");
        assert_eq!(retrieved.mentality, Mentality::Defensive);
    }

    #[tokio::test]
    async fn test_delete_tactics() {
        let pool = setup_test_db().await;
        let repo = TacticsRepository::new(pool);

        let tactics = create_test_tactics();
        let id = repo.create(&tactics).await.unwrap();

        repo.delete(id).await.unwrap();

        let result = repo.get_by_id(id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_by_squad() {
        let pool = setup_test_db().await;
        let repo = TacticsRepository::new(pool);

        let mut tactics1 = create_test_tactics();
        tactics1.squad_id = Some(1);

        let mut tactics2 = create_test_tactics();
        tactics2.name = "Tactics 2".to_string();
        tactics2.squad_id = Some(1);

        let mut tactics3 = create_test_tactics();
        tactics3.name = "Tactics 3".to_string();
        tactics3.squad_id = Some(2);

        repo.create(&tactics1).await.unwrap();
        repo.create(&tactics2).await.unwrap();
        repo.create(&tactics3).await.unwrap();

        let squad1_tactics = repo.get_by_squad(1).await.unwrap();
        assert_eq!(squad1_tactics.len(), 2);
        assert!(squad1_tactics.iter().all(|t| t.squad_id == Some(1)));
    }

    #[tokio::test]
    async fn test_tactics_with_all_settings() {
        let pool = setup_test_db().await;
        let repo = TacticsRepository::new(pool);

        let mut tactics = create_test_tactics();
        tactics.pressing_intensity = PressingIntensity::MuchMore;
        tactics.defensive_line = DefensiveLine::Higher;

        let id = repo.create(&tactics).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.pressing_intensity, PressingIntensity::MuchMore);
        assert_eq!(retrieved.defensive_line, DefensiveLine::Higher);
    }
}
