import type { FullPlantsDto } from "./api";

/**
 * Calculates the number of days between two dates, ignoring time components.
 * @param date1 The first date
 * @param date2 The second date
 * @returns The absolute number of days between the two dates
 */
const calcDayDifference = (date1: Date, date2: Date): number => {
    const d1 = new Date(date1.getFullYear(), date1.getMonth(), date1.getDate());
    const d2 = new Date(date2.getFullYear(), date2.getMonth(), date2.getDate());
    const diffTime = Math.abs(d2.getTime() - d1.getTime());
    return diffTime / (1000 * 60 * 60 * 24);
}

/**
 * Filters plants that need to be watered on a specific date based on their watering interval
 * @param plants - Array of plant objects containing watering information
 * @param date - Date to check for plants that need watering
 * @returns Array of plants that need to be watered on the specified date
 */
export function filterPlantsAtDay(plants: FullPlantsDto[], date: Date): FullPlantsDto[] {
    const plantsAtDay: FullPlantsDto[] = []
    for (const plant of plants) {
        const dayDiff = calcDayDifference(date, plant.lastWatered);
        if (dayDiff % plant.dayInterval === 0) {
            plantsAtDay.push(plant);
        }
    }
    return plantsAtDay;
}