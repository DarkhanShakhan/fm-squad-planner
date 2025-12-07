use sqlx::SqlitePool;
use shared::models::{Squad, SquadPlayer};
use crate::error::{AppError, AppResult};

pub struct SquadRepository {
    pool: SqlitePool,
}

impl SquadRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, squad: &Squad) -> AppResult<i64> {
        let starting_xi_json = serde_json::to_string(&squad.starting_xi)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize starting_xi: {}", e)))?;

        let substitutes_json = serde_json::to_string(&squad.substitutes)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize substitutes: {}", e)))?;

        let result = sqlx::query!(
            r#"
            INSERT INTO squads (name, formation_id, starting_xi, substitutes, average_rating, total_value, total_wage)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            squad.name,
            squad.formation_id,
            starting_xi_json,
            substitutes_json,
            squad.average_rating,
            squad.total_value,
            squad.total_wage
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Squad> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM squads WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let starting_xi: Vec<SquadPlayer> = serde_json::from_str(&r.starting_xi)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse starting_xi: {}", e)))?;

                let substitutes: Vec<i64> = serde_json::from_str(&r.substitutes)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse substitutes: {}", e)))?;

                Ok(Squad {
                    id: Some(r.id),
                    name: r.name,
                    formation_id: r.formation_id,
                    starting_xi,
                    substitutes,
                    average_rating: r.average_rating,
                    total_value: r.total_value,
                    total_wage: r.total_wage,
                })
            }
            None => Err(AppError::NotFound(format!("Squad with id {} not found", id))),
        }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Squad>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM squads ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut squads = Vec::new();
        for r in rows {
            let starting_xi: Vec<SquadPlayer> = serde_json::from_str(&r.starting_xi)
                .map_err(|e| AppError::InternalError(format!("Failed to parse starting_xi: {}", e)))?;

            let substitutes: Vec<i64> = serde_json::from_str(&r.substitutes)
                .map_err(|e| AppError::InternalError(format!("Failed to parse substitutes: {}", e)))?;

            squads.push(Squad {
                id: Some(r.id),
                name: r.name,
                formation_id: r.formation_id,
                starting_xi,
                substitutes,
                average_rating: r.average_rating,
                total_value: r.total_value,
                total_wage: r.total_wage,
            });
        }

        Ok(squads)
    }

    pub async fn update(&self, id: i64, squad: &Squad) -> AppResult<()> {
        let starting_xi_json = serde_json::to_string(&squad.starting_xi)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize starting_xi: {}", e)))?;

        let substitutes_json = serde_json::to_string(&squad.substitutes)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize substitutes: {}", e)))?;

        let result = sqlx::query!(
            r#"
            UPDATE squads SET
                name = ?1,
                formation_id = ?2,
                starting_xi = ?3,
                substitutes = ?4,
                average_rating = ?5,
                total_value = ?6,
                total_wage = ?7
            WHERE id = ?8
            "#,
            squad.name,
            squad.formation_id,
            starting_xi_json,
            substitutes_json,
            squad.average_rating,
            squad.total_value,
            squad.total_wage,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Squad with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM squads WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Squad with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn get_by_formation(&self, formation_id: i64) -> AppResult<Vec<Squad>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM squads WHERE formation_id = ?1 ORDER BY name
            "#,
            formation_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut squads = Vec::new();
        for r in rows {
            let starting_xi: Vec<SquadPlayer> = serde_json::from_str(&r.starting_xi)
                .map_err(|e| AppError::InternalError(format!("Failed to parse starting_xi: {}", e)))?;

            let substitutes: Vec<i64> = serde_json::from_str(&r.substitutes)
                .map_err(|e| AppError::InternalError(format!("Failed to parse substitutes: {}", e)))?;

            squads.push(Squad {
                id: Some(r.id),
                name: r.name,
                formation_id: r.formation_id,
                starting_xi,
                substitutes,
                average_rating: r.average_rating,
                total_value: r.total_value,
                total_wage: r.total_wage,
            });
        }

        Ok(squads)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::{Position, SquadPlayer};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    fn create_test_squad() -> Squad {
        let mut squad = Squad::new("Test Squad".to_string(), 1);

        squad.add_starter(SquadPlayer {
            player_id: 1,
            position: Position::GK,
            tactical_role: None,
            suitability: Some(85.0),
        });

        squad.add_substitute(2);
        squad.add_substitute(3);

        squad
    }

    #[tokio::test]
    async fn test_create_and_get_squad() {
        let pool = setup_test_db().await;
        let repo = SquadRepository::new(pool);

        let squad = create_test_squad();
        let id = repo.create(&squad).await.unwrap();
        assert!(id > 0);

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Test Squad");
        assert_eq!(retrieved.formation_id, 1);
        assert_eq!(retrieved.starting_xi.len(), 1);
        assert_eq!(retrieved.substitutes.len(), 2);
    }

    #[tokio::test]
    async fn test_get_all_squads() {
        let pool = setup_test_db().await;
        let repo = SquadRepository::new(pool);

        let squad1 = create_test_squad();
        let mut squad2 = create_test_squad();
        squad2.name = "Squad 2".to_string();

        repo.create(&squad1).await.unwrap();
        repo.create(&squad2).await.unwrap();

        let squads = repo.get_all().await.unwrap();
        assert_eq!(squads.len(), 2);
    }

    #[tokio::test]
    async fn test_update_squad() {
        let pool = setup_test_db().await;
        let repo = SquadRepository::new(pool);

        let squad = create_test_squad();
        let id = repo.create(&squad).await.unwrap();

        let mut updated_squad = create_test_squad();
        updated_squad.name = "Updated Squad".to_string();
        updated_squad.formation_id = 2;

        repo.update(id, &updated_squad).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Updated Squad");
        assert_eq!(retrieved.formation_id, 2);
    }

    #[tokio::test]
    async fn test_delete_squad() {
        let pool = setup_test_db().await;
        let repo = SquadRepository::new(pool);

        let squad = create_test_squad();
        let id = repo.create(&squad).await.unwrap();

        repo.delete(id).await.unwrap();

        let result = repo.get_by_id(id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_by_formation() {
        let pool = setup_test_db().await;
        let repo = SquadRepository::new(pool);

        let mut squad1 = create_test_squad();
        squad1.formation_id = 1;

        let mut squad2 = create_test_squad();
        squad2.name = "Squad 2".to_string();
        squad2.formation_id = 1;

        let mut squad3 = create_test_squad();
        squad3.name = "Squad 3".to_string();
        squad3.formation_id = 2;

        repo.create(&squad1).await.unwrap();
        repo.create(&squad2).await.unwrap();
        repo.create(&squad3).await.unwrap();

        let formation1_squads = repo.get_by_formation(1).await.unwrap();
        assert_eq!(formation1_squads.len(), 2);
        assert!(formation1_squads.iter().all(|s| s.formation_id == 1));
    }

    #[tokio::test]
    async fn test_squad_with_statistics() {
        let pool = setup_test_db().await;
        let repo = SquadRepository::new(pool);

        let mut squad = create_test_squad();
        squad.average_rating = Some(75.5);
        squad.total_value = Some(50000000);
        squad.total_wage = Some(250000);

        let id = repo.create(&squad).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.average_rating, Some(75.5));
        assert_eq!(retrieved.total_value, Some(50000000));
        assert_eq!(retrieved.total_wage, Some(250000));
    }
}
