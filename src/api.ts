import { invoke } from "@tauri-apps/api/core";
import type { PlantDto } from "./interfaces/dto";
import type { PlantModel, PlantWithWateringsModel } from "./interfaces/models";

const formatDate = (date: Date): string => {
    return date.toISOString()
        .replace('T', ' ')
        .split('.')[0];
}

const parseNaiveDateTime = (dateStr: string): Date => {
    return new Date(dateStr.replace(' ', 'T') + 'Z');
}

class Api {
    async getPlantsWithRecentWatering(date: Date, offset: number): Promise<PlantWithWateringsModel[]> {
        const response = await invoke<PlantWithWateringsModel[]>('get_all_plant_with_watering', {
            date: formatDate(date),
            offset: offset
        });
        console.log(response);

        return response.map(plant => ({
            ...plant,
            waterings: plant.waterings.map((watering: any) => ({
                ...watering,
                dateWatered: parseNaiveDateTime(watering.dateWatered)
            }))
        }));
    }

    async addPlant(plant: PlantDto): Promise<PlantModel> {
        return await invoke<PlantModel>('add_plant', { plantDto: plant });
    }

    async getAllPlants(): Promise<PlantModel[]> {
        return await invoke<PlantModel[]>('get_all_plants');
    }

    async addWatering(plantId: number, date: Date): Promise<void> {
        return await invoke('add_watering', {
            plantId: plantId,
            date: formatDate(date)
        })
    }
}

export const api = new Api();