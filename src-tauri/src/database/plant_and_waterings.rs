use super::{BackendError, Plant, Watering};
use chrono::{Days, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct PlantWithWaterings {
    pub id: i64,
    pub name: String,
    #[serde(rename = "dayInterval")]
    pub day_interval: i64,
    #[serde(rename = "waterQuantity")]
    pub water_quantity: i64,
    pub image: Option<String>,
    pub waterings: Vec<Watering>,
}

impl PlantWithWaterings {
    pub fn new(plant: Plant, waterings: Vec<Watering>) -> Self {
        Self {
            id: plant.id,
            name: plant.name,
            day_interval: plant.day_interval,
            water_quantity: plant.water_quantity,
            image: plant.image,
            waterings,
        }
    }

    pub async fn new_from_plant_id(pool: &SqlitePool, id: i64) -> Result<Self, BackendError> {
        // Get plant
        let plant = Plant::get_plant_by_id(&pool, id).await?;

        // Get watering from this plant
        let waterings = Watering::get_watering_by_plant_id(&pool, id).await?;

        Ok(Self::new(plant, waterings))
    }

    pub async fn get_plants_with_recent_waterings(
        pool: &SqlitePool,
        date_end_opt: Option<NaiveDateTime>,
        day_offset_opt: Option<u64>,
    ) -> Result<Vec<Self>, BackendError> {
        // Handle optional date_end
        let date_end = date_end_opt.unwrap_or(Local::now().naive_utc());
        let day_offset = day_offset_opt.unwrap_or(31);

        // Start date is 31 days before end
        // TODO, need to return a custom error for all the functions of the db
        let date_start = date_end.checked_sub_days(Days::new(day_offset)).unwrap();

        // Fetch watering result for 31 past days
        let waterings = Watering::get_watering_by_dates(&pool, date_start, date_end).await?;

        // Fetch plants
        let plants =
            Plant::get_plant_by_ids(&pool, waterings.iter().map(|w| w.plant_id).collect()).await?;

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
