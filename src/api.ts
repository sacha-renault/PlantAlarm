import { invoke } from "@tauri-apps/api/core";
import type { PlantDto } from "./interfaces/dto";
import type { PlantModel } from "./interfaces/models";

export interface FullPlantsDto {
    id: number;
    name: string;
    lastWatered: Date;
    img: string | null;
    dayInterval: number;
    waterQuantity: number;
}

const mockedPlants = [
    {
        id: 0,
        name: "Orchidée 1",
        lastWatered: new Date(Date.now()),
        dayInterval: 2,
        img: null,
        waterQuantity: 200
    },
    {
        id: 1,
        name: "Orchidée 2",
        lastWatered: new Date(Date.now()),
        dayInterval: 4,
        img: "https://townhouseflowers.com.au/wp-content/uploads/2022/04/phally.jpg.webp",
        waterQuantity: 200
    },
]

class Api {
    async getAllPlants(): Promise<FullPlantsDto[]> {
        return mockedPlants;
    }

    async addPlant(plant: PlantDto): Promise<number> {
        return await invoke('add_plant', { plantDto: plant });
    }
}

export const api = new Api();