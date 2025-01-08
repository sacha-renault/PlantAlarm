import type { PlantDto } from "./dto";

export interface PlantModel extends PlantDto {
    id: number;
}

export interface WateringModel {
    id: number;
    plantId: number;
    dateWatered: Date;
}

export interface PlantWithWateringsModel extends PlantModel {
    waterings: WateringModel[];
}
