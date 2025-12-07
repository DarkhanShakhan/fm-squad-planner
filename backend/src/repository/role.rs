use sqlx::SqlitePool;
use shared::models::Role;
use crate::error::{AppError, AppResult};
use std::collections::HashMap;

pub struct RoleRepository {
    pool: SqlitePool,
}

impl RoleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, role: &Role) -> AppResult<i64> {
        let attribute_weights_json = serde_json::to_string(&role.attribute_weights)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize attribute_weights: {}", e)))?;

        let result = sqlx::query!(
            r#"
            INSERT INTO roles (name, position, duty, description, attribute_weights, is_custom)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            role.name,
            role.position.as_str(),
            role.duty.duty_name(),
            role.description,
            attribute_weights_json,
            role.is_custom
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<Role> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM roles WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let position = r.position.parse()
                    .map_err(|_| AppError::InternalError(format!("Invalid position: {}", r.position)))?;

                let duty = match r.duty.as_str() {
                    "Defend" => shared::models::Duty::Defend,
                    "Support" => shared::models::Duty::Support,
                    "Attack" => shared::models::Duty::Attack,
                    "Automatic" => shared::models::Duty::Automatic,
                    _ => return Err(AppError::InternalError(format!("Invalid duty: {}", r.duty))),
                };

                let attribute_weights: HashMap<String, f32> = serde_json::from_str(&r.attribute_weights)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse attribute_weights: {}", e)))?;

                Ok(Role {
                    id: Some(r.id),
                    name: r.name,
                    position,
                    duty,
                    description: r.description,
                    attribute_weights,
                    is_custom: r.is_custom != 0,
                })
            }
            None => Err(AppError::NotFound(format!("Role with id {} not found", id))),
        }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Role>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM roles ORDER BY is_custom, name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut roles = Vec::new();
        for r in rows {
            let position = r.position.parse()
                .map_err(|_| AppError::InternalError(format!("Invalid position: {}", r.position)))?;

            let duty = match r.duty.as_str() {
                "Defend" => shared::models::Duty::Defend,
                "Support" => shared::models::Duty::Support,
                "Attack" => shared::models::Duty::Attack,
                "Automatic" => shared::models::Duty::Automatic,
                _ => return Err(AppError::InternalError(format!("Invalid duty: {}", r.duty))),
            };

            let attribute_weights: HashMap<String, f32> = serde_json::from_str(&r.attribute_weights)
                .map_err(|e| AppError::InternalError(format!("Failed to parse attribute_weights: {}", e)))?;

            roles.push(Role {
                id: Some(r.id),
                name: r.name,
                position,
                duty,
                description: r.description,
                attribute_weights,
                is_custom: r.is_custom != 0,
            });
        }

        Ok(roles)
    }

    pub async fn update(&self, id: i64, role: &Role) -> AppResult<()> {
        let attribute_weights_json = serde_json::to_string(&role.attribute_weights)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize attribute_weights: {}", e)))?;

        let result = sqlx::query!(
            r#"
            UPDATE roles SET
                name = ?1,
                position = ?2,
                duty = ?3,
                description = ?4,
                attribute_weights = ?5,
                is_custom = ?6
            WHERE id = ?7
            "#,
            role.name,
            role.position.as_str(),
            role.duty.duty_name(),
            role.description,
            attribute_weights_json,
            role.is_custom,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Role with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM roles WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Role with id {} not found", id)));
        }

        Ok(())
    }

    pub async fn get_by_position(&self, position: &str) -> AppResult<Vec<Role>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM roles WHERE position = ?1 ORDER BY name
            "#,
            position
        )
        .fetch_all(&self.pool)
        .await?;

        let mut roles = Vec::new();
        for r in rows {
            let position = r.position.parse()
                .map_err(|_| AppError::InternalError(format!("Invalid position: {}", r.position)))?;

            let duty = match r.duty.as_str() {
                "Defend" => shared::models::Duty::Defend,
                "Support" => shared::models::Duty::Support,
                "Attack" => shared::models::Duty::Attack,
                "Automatic" => shared::models::Duty::Automatic,
                _ => return Err(AppError::InternalError(format!("Invalid duty: {}", r.duty))),
            };

            let attribute_weights: HashMap<String, f32> = serde_json::from_str(&r.attribute_weights)
                .map_err(|e| AppError::InternalError(format!("Failed to parse attribute_weights: {}", e)))?;

            roles.push(Role {
                id: Some(r.id),
                name: r.name,
                position,
                duty,
                description: r.description,
                attribute_weights,
                is_custom: r.is_custom != 0,
            });
        }

        Ok(roles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::{Position, Duty};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    fn create_test_role() -> Role {
        let mut attribute_weights = HashMap::new();
        attribute_weights.insert("finishing".to_string(), 0.9);
        attribute_weights.insert("pace".to_string(), 0.8);

        Role::new(
            "Test Striker".to_string(),
            Position::STC,
            Duty::Attack,
            attribute_weights,
        )
    }

    #[tokio::test]
    async fn test_create_and_get_role() {
        let pool = setup_test_db().await;
        let repo = RoleRepository::new(pool);

        let role = create_test_role();
        let id = repo.create(&role).await.unwrap();
        assert!(id > 0);

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Test Striker");
        assert_eq!(retrieved.position, Position::STC);
        assert_eq!(retrieved.duty, Duty::Attack);
        assert_eq!(retrieved.get_weight("finishing"), 0.9);
    }

    #[tokio::test]
    async fn test_get_all_roles() {
        let pool = setup_test_db().await;
        let repo = RoleRepository::new(pool);

        let role1 = create_test_role();
        let mut role2 = create_test_role();
        role2.name = "Test Midfielder".to_string();

        repo.create(&role1).await.unwrap();
        repo.create(&role2).await.unwrap();

        let roles = repo.get_all().await.unwrap();
        assert!(roles.len() >= 2);
    }

    #[tokio::test]
    async fn test_update_role() {
        let pool = setup_test_db().await;
        let repo = RoleRepository::new(pool);

        let role = create_test_role();
        let id = repo.create(&role).await.unwrap();

        let mut updated_role = create_test_role();
        updated_role.name = "Updated Striker".to_string();
        updated_role.description = Some("Updated description".to_string());

        repo.update(id, &updated_role).await.unwrap();

        let retrieved = repo.get_by_id(id).await.unwrap();
        assert_eq!(retrieved.name, "Updated Striker");
        assert_eq!(retrieved.description, Some("Updated description".to_string()));
    }

    #[tokio::test]
    async fn test_delete_role() {
        let pool = setup_test_db().await;
        let repo = RoleRepository::new(pool);

        let role = create_test_role();
        let id = repo.create(&role).await.unwrap();

        repo.delete(id).await.unwrap();

        let result = repo.get_by_id(id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_by_position() {
        let pool = setup_test_db().await;
        let repo = RoleRepository::new(pool);

        let role1 = create_test_role();
        let mut role2 = create_test_role();
        role2.name = "Another Striker".to_string();

        let mut weights = HashMap::new();
        weights.insert("passing".to_string(), 0.9);
        let role3 = Role::new(
            "Midfielder".to_string(),
            Position::MC,
            Duty::Support,
            weights,
        );

        repo.create(&role1).await.unwrap();
        repo.create(&role2).await.unwrap();
        repo.create(&role3).await.unwrap();

        let strikers = repo.get_by_position("STC").await.unwrap();
        assert_eq!(strikers.len(), 2);
        assert!(strikers.iter().all(|r| r.position == Position::STC));
    }
}
