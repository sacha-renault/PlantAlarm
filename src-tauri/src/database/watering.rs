use super::{constant::DB_RESULT_LIMIT, BackendError, MapErrorExt};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use sqlx::{Error, Executor};

#[derive(Debug, FromRow, Serialize)]
pub struct Watering {
    pub id: i64,
    pub plant_id: i64,
    pub date_watered: NaiveDateTime,
}

impl Watering {
    /// Insert a new watering entry
    pub async fn insert_watering(
        pool: &SqlitePool,
        plant_id: i64,
        date_watered: NaiveDateTime,
    ) -> Result<(), BackendError> {
        // Check if the plant exists
        let plant_exists_query = r#"SELECT COUNT(*) FROM plant WHERE id = ?"#;
        let plant_exists: (i64,) = sqlx::query_as(plant_exists_query)
            .bind(plant_id)
            .fetch_one(pool)
            .await
            .map_error()?;

        if plant_exists.0 == 0 {
            return Err(BackendError::DatabaseError(
                "Cannot insert watering to a plant that doesn't exist".to_string(),
            )); // Or a custom error indicating the plant does not exist
        }

        // If the plant exists, proceed to insert the watering entry
        let insert_query = r#"
            INSERT INTO watering (plant_id, date_watered)
            VALUES (?, ?)
        "#;

        sqlx::query(insert_query)
            .bind(plant_id)
            .bind(date_watered)
            .execute(pool)
            .await
            .map_error()?;

        Ok(())
    }

    /// Insert a new watering entry at date now
    pub async fn insert_watering_now(pool: &SqlitePool, plant_id: i64) -> Result<(), BackendError> {
        // Init a time at now
        let now = Local::now().naive_utc();

        // Insert into db
        Self::insert_watering(&pool, plant_id, now).await?;

        Ok(())
    }

    /// Get all watering in page
    pub async fn get_watering_by_page(
        pool: &SqlitePool,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Watering>, BackendError> {
        // Calc begenning and end
        let begin = page * page_size;

        // Make the query
        let query = r#"
            SELECT *
            FROM watering
            ORDER BY date_watered DESC
            LIMIT ?1 OFFSET ?2
        "#;

        // Fetch the results from the database
        let waterings = sqlx::query_as::<_, Watering>(query)
            .bind(begin + page_size)
            .bind(begin)
            .fetch_all(pool)
            .await
            .map_error()?;

        Ok(waterings)
    }

    /// Get watering by date
    pub async fn get_watering_by_dates(
        pool: &SqlitePool,
        date_start: NaiveDateTime,
        date_end: NaiveDateTime,
    ) -> Result<Vec<Watering>, BackendError> {
        // assert!(date_start < date_end, "Start date should be more than end date");

        // Make the query
        let query = r#"
            SELECT *
            FROM watering
            WHERE date_watered >= ?1
            AND date_watered <= ?2
        "#;

        // Fetch the results from the database
        let waterings = sqlx::query_as::<_, Watering>(query)
            .bind(date_start)
            .bind(date_end)
            .fetch_all(pool)
            .await
            .map_error()?;

        Ok(waterings)
    }

    /// Get waterings by plant id, limit the result to 1k result (not useful to have more)
    pub async fn get_watering_by_plant_id(
        pool: &SqlitePool,
        plant_id: i64,
    ) -> Result<Vec<Watering>, BackendError> {
        // Make the query
        let query = r#"
            SELECT *
            FROM watering
            WHERE plant_id = ?1
            ORDER BY date_watered DESC
            LIMIT ?2
        "#;

        // Fetch the results from the database
        let waterings = sqlx::query_as::<_, Watering>(query)
            .bind(plant_id)
            .bind(DB_RESULT_LIMIT)
            .fetch_all(pool)
            .await
            .map_error()?;

        Ok(waterings)
    }
}
