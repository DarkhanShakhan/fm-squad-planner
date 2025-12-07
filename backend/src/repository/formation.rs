use sqlx::SqlitePool;
use shared::models::{Formation, FormationPosition};
use crate::error::{AppError, AppResult};

pub struct FormationRepository {
    pool: SqlitePool,
}

impl FormationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, formation: &Formation) -> AppResult<i64> {
        let positions_json = serde_json::to_string(&formation.positions)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize positions: {}", e)))?;

        let result = sqlx::query!(
            r#"
            INSERT INTO formations (name, description, positions, is_custom)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            formation.name,
            formation.description,
            positions_json,
            formation.is_custom
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Formation> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM formations WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let positions: Vec<FormationPosition> = serde_json::from_str(&r.positions)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse positions: {}", e)))?;

                Ok(Formation {
                    id: Some(r.id),
                    name: r.name,
                    description: r.description,
                    positions,
                    is_custom: r.is_custom != 0,
                })
            }
            None => Err(AppError::NotFound(format!("Formation with id {} not found", id))),
        }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Formation>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM formations ORDER BY is_custom, name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut formations = Vec::new();
        for r in rows {
            let positions: Vec<FormationPosition> = serde_json::from_str(&r.positions)
                .map_err(|e| AppError::InternalError(format!("Failed to parse positions: {}", e)))?;

            formations.push(Formation {
                id: Some(r.id),
                name: r.name,
                description: r.description,
                positions,
                is_custom: r.is_custom != 0,
            });
        }

        Ok(formations)
    }

    pub async fn update(&self, id: i64, formation: &Formation) -> AppResult<()> {
        let positions_json = serde_json::to_string(&formation.positions)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize positions: {}", e)))?;

        let result = sqlx::query!(
            r#"
            UPDATE formations SET
                name = ?1,
                description = ?2,
                positions = ?3,
                is_custom = ?4
            WHERE id = ?5
            "#,
            formation.name,
            formation.description,
            positions_json,
            formation.is_custom,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Formation with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM formations WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Formation with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn get_custom_formations(&self) -> AppResult<Vec<Formation>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM formations WHERE is_custom = 1 ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut formations = Vec::new();
        for r in rows {
            let positions: Vec<FormationPosition> = serde_json::from_str(&r.positions)
                .map_err(|e| AppError::InternalError(format!("Failed to parse positions: {}", e)))?;

            formations.push(Formation {
                id: Some(r.id),
                name: r.name,
                description: r.description,
                positions,
                is_custom: r.is_custom != 0,
            });
        }

        Ok(formations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::{Position, FormationPosition};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    fn create_test_formation() -> Formation {
        let positions = vec![
            FormationPosition {
                position: Position::GK,
                x: 50.0,
                y: 5.0,
                tactical_role: None,
            },
            FormationPosition {
                position: Position::DC,
                x: 50.0,
                y: 20.0,
                tactical_role: None,
            },
        ];

        Formation::new("Test Formation".to_string(), positions)
    }

    #[tokio::test]
    async fn test_create_and_get_formation() {
        let pool = setup_test_db().await;
        let repo = FormationRepository::new(pool);

        let formation = create_test_formation();
        let id = repo.create(&formation).await.unwrap();
        assert!(id > 0);

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Test Formation");
        assert_eq!(retrieved.positions.len(), 2);
        assert!(!retrieved.is_custom);
    }

    #[tokio::test]
    async fn test_get_all_formations() {
        let pool = setup_test_db().await;
        let repo = FormationRepository::new(pool);

        let formation1 = create_test_formation();
        let mut formation2 = create_test_formation();
        formation2.name = "Formation 2".to_string();

        repo.create(&formation1).await.unwrap();
        repo.create(&formation2).await.unwrap();

        let formations = repo.get_all().await.unwrap();
        assert!(formations.len() >= 2);
    }

    #[tokio::test]
    async fn test_update_formation() {
        let pool = setup_test_db().await;
        let repo = FormationRepository::new(pool);

        let formation = create_test_formation();
        let id = repo.create(&formation).await.unwrap();

        let mut updated_formation = create_test_formation();
        updated_formation.name = "Updated Formation".to_string();
        updated_formation.description = Some("Updated description".to_string());

        repo.update(id, &updated_formation).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Updated Formation");
        assert_eq!(retrieved.description, Some("Updated description".to_string()));
    }

    #[tokio::test]
    async fn test_delete_formation() {
        let pool = setup_test_db().await;
        let repo = FormationRepository::new(pool);

        let formation = create_test_formation();
        let id = repo.create(&formation).await.unwrap();

        repo.delete(id).await.unwrap();

        let result = repo.get_by_id(id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_custom_formations() {
        let pool = setup_test_db().await;
        let repo = FormationRepository::new(pool);

        let mut formation1 = create_test_formation();
        let mut formation2 = create_test_formation();
        formation2.name = "Custom Formation".to_string();
        formation2.is_custom = true;

        repo.create(&formation1).await.unwrap();
        repo.create(&formation2).await.unwrap();

        let custom_formations = repo.get_custom_formations().await.unwrap();
        assert!(custom_formations.iter().all(|f| f.is_custom));
    }

    #[tokio::test]
    async fn test_formation_validation() {
        let pool = setup_test_db().await;
        let repo = FormationRepository::new(pool);

        let positions = vec![
            FormationPosition {
                position: Position::GK,
                x: 50.0,
                y: 5.0,
                tactical_role: None,
            },
        ];

        let formation = Formation::new("Invalid Formation".to_string(), positions);
        assert!(!formation.is_valid());

        repo.create(&formation).await.unwrap();
    }
}
