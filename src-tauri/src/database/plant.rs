use std::collections::HashMap;
use chrono::{NaiveDateTime, Local, Days};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Error};
use sqlx::sqlite::SqlitePool;
use super::Watering;

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
    pub async fn get_plants(pool: &SqlitePool) -> Result<Vec<Self>, Error> {
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
    pub async fn get_plant_by_id(pool: &SqlitePool, id: i64) -> Result<Self, Error> {
        // Make the query
        let query = r#"
            SELECT * FROM plant
            WHERE id = ?1
        "#;

        // Fetch the results from the database
        let plant = sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_one(pool) // We expect at most one result
            .await?;

        Ok(plant)
    }

    /// get a single plant by its name
    pub async fn get_plant_by_name(pool: &SqlitePool, name: String) -> Result<Self, Error> {
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
        // Make the query with WHERE IN clause
        let query = r#"
            SELECT * FROM plant
            WHERE id IN (
                SELECT value
                FROM json_each(?1)
            )
            ORDER BY id
        "#;

        // SQLite doesn't have native array support, so we serialize ids to JSON
        let ids_json = serde_json::to_string(&ids).expect("Couldn't serialize the ids");

        // Fetch the results from the database
        let plants = sqlx::query_as::<_, Plant>(query)
            .bind(ids_json)
            .fetch_all(pool)
            .await?;

        Ok(plants)
    }
}

#[derive(Debug, Serialize)]
pub struct PlantWithWaterings {
    pub name: String,
    pub day_interval: i64,
    pub water_quantity: i64,
    pub image: Option<String>,
    pub waterings: Vec<Watering>
}

impl PlantWithWaterings {
    pub fn new(plant: Plant, waterings: Vec<Watering>) -> Self {
        Self {
            name: plant.name,
            day_interval: plant.day_interval,
            water_quantity: plant.water_quantity,
            image: plant.image,
            waterings,
        }
    }

    pub async fn new_from_plant_id(pool: &SqlitePool, id: i64) -> Result<Self, Error> {
        // Get plant
        let plant = Plant::get_plant_by_id(&pool, id).await?;

        // Get watering from this plant
        let waterings = Watering::get_watering_by_plant_id(&pool, id).await?;

        Ok(Self::new(plant, waterings))
    }

    pub async fn get_plants_with_watering_month(pool: &SqlitePool, date_end_opt: Option<NaiveDateTime>, day_offset_opt: Option<u64>) -> Result<Vec<Self>, Error> {
        // Handle optional date_end
        let date_end = date_end_opt.unwrap_or(Local::now().naive_utc());
        let day_offset = day_offset_opt.unwrap_or(31);

        // Start date is 31 days before end
        // TODO, need to return a custom error for all the functions of the db
        let date_start = date_end.checked_sub_days(Days::new(day_offset)).unwrap();

        // Fetch watering result for 31 past days
        let waterings = Watering::get_watering_by_dates(&pool, date_start, date_end).await?;

        // Fetch plants
        let plants = Plant::get_plant_by_ids(&pool, waterings.iter().map(|w| w.plant_id).collect()).await?;

        // Group waterings by `plant_id`
        let mut watering_map: HashMap<i64, Vec<Watering>> = HashMap::new();
        for watering in waterings {
            watering_map
                .entry(watering.plant_id)
                .or_insert_with(Vec::new)
                .push(watering);
        }

        // group plants to watering
        let plants_with_waterings: Vec<PlantWithWaterings> = plants
            .into_iter()
            .map(|plant| {
                let associated_waterings = watering_map.remove(&plant.id).unwrap_or_default();
                PlantWithWaterings::new(plant, associated_waterings)
            })
            .collect();

        Ok(plants_with_waterings)
    }
}
