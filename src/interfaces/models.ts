import type { PlantDto } from "./dto";

export interface PlantModel extends PlantDto {
    id: number;
}

export interface PlantWithWateringsModel extends PlantModel {
    lastWatered: Date;
}
