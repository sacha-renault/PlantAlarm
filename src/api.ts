import { invoke } from "@tauri-apps/api/core";
import type { PlantDto } from "./interfaces/dto";
import type { PlantModel, PlantWithWateringsModel } from "./interfaces/models";


const mockedPlants = [
    {
        id: 0,
        name: "Orchidée 1",
        lastWatered: new Date(Date.now()),
        dayInterval: 2,
        image: null,
        waterQuantity: 200
    },
    {
        id: 1,
        name: "Orchidée 2",
        lastWatered: new Date(Date.now()),
        dayInterval: 4,
        image: "https://townhouseflowers.com.au/wp-content/uploads/2022/04/phally.jpg.webp",
        waterQuantity: 200
    },
]

class Api {
    async getPlantsWithRecentWatering(): Promise<PlantWithWateringsModel[]> {
        return mockedPlants;
    }

    // async getPlantWithNextWatering(): Promise<>

    async addPlant(plant: PlantDto): Promise<PlantModel> {
        return await invoke<PlantModel>('add_plant', { plantDto: plant });
    }

    async getAllPlants(): Promise<PlantModel[]> {
        return await invoke<PlantModel[]>('get_all_plants');
    }
}

export const api = new Api();