use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Error};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Deserialize)]
pub struct PlantDto {
    pub name: String,
    pub day_interval: i64,
    pub water_quantity: i64,
    pub image: Option<String>
}

#[derive(Debug, FromRow, Serialize)]
pub struct Plant {
    pub id: i64,
    pub name: String,
    pub day_interval: i64,
    pub water_quantity: i64,
    pub image: Option<String>
}

impl Plant {
    /// Get all plants in the db
    pub async fn get_plants(pool: &SqlitePool) -> Result<Vec<Plant>, Error> {
        // Make the query
        let query = r#"
            SELECT * FROM plant
        "#;

        // Fetch the results from the database
        let plants = sqlx::query_as::<_, Plant>(query)
            .fetch_all(pool)
            .await?;

        Ok(plants)
    }

    /// insert a new plant
    pub async fn insert_plant(pool: &SqlitePool, plant: PlantDto) -> Result<(), Error> {
        // Make the insert query
        let query = r#"
            INSERT INTO plant (name, day_interval, water_quantity, image)
            VALUES (?, ?, ?, ?)
        "#;

        // Execute the insert query
        sqlx::query(query)
            .bind(&plant.name)
            .bind(plant.day_interval)
            .bind(plant.water_quantity)
            .bind(plant.image.as_ref())
            .execute(pool)
            .await?;

        Ok(())
    }

    /// get a single plant by its id
    pub async fn get_plant_by_id(pool: &SqlitePool, id: i64) -> Result<Plant, Error> {
        // Make the query
        let query = r#"
            SELECT * FROM plant
            WHERE id = ?1
        "#;

        // Fetch the results from the database
        let plant = sqlx::query_as::<_, Plant>(query)
            .bind(id)
            .fetch_one(pool) // We expect at most one result
            .await?;

        Ok(plant)
    }

    /// get a single plant by its name
    pub async fn get_plant_by_name(pool: &SqlitePool, name: String) -> Result<Plant, Error> {
        // Make the query
        let query = r#"
            SELECT * FROM plant
            WHERE name = ?1
        "#;

        // Fetch the results from the database
        let plant = sqlx::query_as::<_, Plant>(query)
            .bind(name)
            .fetch_one(pool) // We expect at most one result
            .await?;

        Ok(plant)
    }

    /// Get the plants given to an vec of ids
    pub async fn get_plant_by_ids(pool: &SqlitePool, ids: Vec<i64>) -> Result<Vec<Plant>, Error> {
        // Make the query with
        let query = r#"
            SELECT * FROM plant
            WHERE id IN (
                SELECT value
                FROM json_each(?1)
            )
            ORDER BY id
        "#;

        // SQLite can't handle rust vector => we have to convert to a json
        let ids_json = serde_json::to_string(&ids).expect("Couldn't serialize the ids");

        // Fetch the results from the database
        let plants = sqlx::query_as::<_, Plant>(query)
            .bind(ids_json)
            .fetch_all(pool)
            .await?;

        Ok(plants)
    }
}