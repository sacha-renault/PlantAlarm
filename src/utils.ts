import type { PlantWithWateringsModel } from "./interfaces/models";

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

export function isSameDay(date1: Date, date2: Date): boolean {
    return calcDayDifference(date1, date2) === 0;
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
export function filterPlantsAtDay(plants: PlantWithWateringsModel[], date: Date): PlantWithWateringsModel[] {
    const plantsAtDay: PlantWithWateringsModel[] = []
    for (const plant of plants) {
        const lastWatered = plant.waterings.map(w => w.dateWatered).reduce((a, b) => a > b ? a : b);
        const dayDiff = calcDayDifference(date, lastWatered);
        if (dayDiff === 0) {
            plantsAtDay.push(plant);
        }
    }
    return plantsAtDay;
}

// Separate utility function
export function getLastWatered(plant: PlantWithWateringsModel): Date {
    return plant.waterings
        .map((w) => w.dateWatered)
        .reduce((a, b) => (a > b ? a : b));
}

export function groupPlantPerNextWateringDay(plants: PlantWithWateringsModel[]) {
    // Initialize an empty array to store the grouped plants by next watering day
    const groupedPlants: { date: Date, plants: PlantWithWateringsModel[] }[] = [];

    // Iterate through each plant and group them by their next watering day
    plants.forEach((plant) => {
        // Find the last watering date by mapping and reducing the waterings
        const lastWatered = getLastWatered(plant);

        // Calculate the next watering day based on the day interval and reset time to 00:00
        const nextWatering = new Date(
            addDays(lastWatered, plant.dayInterval).setHours(0, 0, 0, 0)
        );

        // Find if the nextWatering date already exists in groupedPlants
        const existingGroup = groupedPlants.find(group => group.date.getTime() === nextWatering.getTime());

        if (existingGroup) {
            // If the date exists, add the plant to the corresponding group
            existingGroup.plants.push(plant);
        } else {
            // If the date doesn't exist, create a new group with this date and the plant
            groupedPlants.push({ date: nextWatering, plants: [plant] });
        }
    });

    // Sort the grouped plants by date in ascending order
    groupedPlants.sort((a, b) => a.date.getTime() - b.date.getTime());

    // Return the grouped plants with their next watering date
    return groupedPlants;
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

/**
 * Formats a given date into a human-readable string based on its relation to the current date.
 *
 * @param {Date} date - The date to format.
 * @returns {string} A formatted string representing the date:
 * - "Today" if the date is today.
 * - "Tomorrow" if the date is tomorrow.
 * - "Yesterday" if the date is yesterday.
 * - "Next [weekday]" if the date is within the next 7 days and in the future.
 * - "Last [weekday]" if the date is within the last 7 days and in the past.
 * - "On dd/MM" if the date is more than 7 days away in either direction.
 */
export function formatDayString(date: Date): string {
    const now = new Date(Date.now());
    const dayDiff = calcDayDifference(date, now);
    const isFuture = now < date;

    if (dayDiff === 0) {
        return 'Today';
    } else if (dayDiff === 1 && isFuture) {
        return 'Tomorrow';
    } else if (dayDiff === 1) {
        return 'Yesterday';
    }

    const dayString = date?.toLocaleDateString('en-US', { weekday: 'long' });
    const dateString = date?.toLocaleString('en-UK', { day: '2-digit', month: '2-digit' }); // UK for day / month
    const prefix = isFuture ? 'Next ' : 'Last ';

    if (dayDiff <= 7) {
        return prefix + dayString; // Next Tuesday or Last Tuesday
    } else {
        return 'On ' + dateString; // On 25/12 or similar
    }
}