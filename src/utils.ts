import type { FullPlantsDto } from "./api";

/**
 * Calculates the number of days between two dates, ignoring time components.
 * @param date1 The first date
 * @param date2 The second date
 * @returns The absolute number of days between the two dates
 */
export const calcDayDifference = (date1: Date, date2: Date): number => {
    const d1 = new Date(date1.getFullYear(), date1.getMonth(), date1.getDate());
    const d2 = new Date(date2.getFullYear(), date2.getMonth(), date2.getDate());
    const diffTime = Math.abs(d2.getTime() - d1.getTime());
    return diffTime / (1000 * 60 * 60 * 24);
}


/**
 * Formats a date object into a string with weekday, day, and month.
 * @param date - The date to format. Can be a Date object or any value that can be converted to a date.
 * @returns A string representation of the date in the format "Weekday, Month Day" (e.g., "Monday, January 1").
 * @example
 * // Returns "Monday, January 1"
 * formatDateWithWeekday(new Date(2024, 0, 1));
 */
export const formatDateWithWeekday = (date: Date | any): string => {
    const options: Intl.DateTimeFormatOptions = {
        weekday: 'long',  // Full name of the weekday
        day: 'numeric',   // Day of the month
        month: 'long',    // Full name of the month
    };
    return date?.toLocaleDateString('en-US', options);
};

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

/**
 * Adds a specified number of days to a given date
 * @param date The starting date
 * @param days Number of days to add
 * @returns A new Date object with the days added
 */
export function addDays(date: Date, days: number): Date {
    const result = new Date(date);
    result.setDate(result.getDate() + days);
    return result;
}