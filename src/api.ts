export interface FullPlantsDto {
    id: number;
    name: string;
    lastWatered: Date;
    img: string | null;
    dayInterval: number;
}

const mockedPlants = [
    {
        id: 0,
        name: "Orchidée 1",
        lastWatered: new Date(Date.now()),
        dayInterval: 2,
        img: null
    },
    {
        id: 1,
        name: "Orchidée 2",
        lastWatered: new Date(Date.now()),
        dayInterval: 4,
        img: "https://townhouseflowers.com.au/wp-content/uploads/2022/04/phally.jpg.webp"
    },
]

class Api {
    async getAllPlants(): Promise<FullPlantsDto[]> {
        return mockedPlants;
    }
}

export const api = new Api();