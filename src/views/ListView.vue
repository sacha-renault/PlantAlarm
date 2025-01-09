<template>
    <div>
        <h1>
            Plants to water
        </h1>
        <n-divider />
        <n-collapse-transition v-for="row in groupedPerDay" :show="row.visibleCount !== 0">
            <n-flex vertical class="wf" align="center">
                <h2> {{ formatDayString(row.date) }} </h2>
                <n-flex class="wf" style="gap: 0px;" v-if="row.plants.length !== 0">
                    <plant-item-list v-for="plant in row.plants" :plant="plant" :date="row.date" :key="plant.id"
                        @plant-watered="handleWatered" />
                </n-flex>
                <n-divider class="list-divider" />
            </n-flex>
        </n-collapse-transition>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { addDays, isSameDay, formatDayString, groupPlantPerNextWateringDay } from '../utils';
import { api } from '../api';
import PlantItemList from '../components/PlantItemList.vue';
import type { PlantWithWateringsModel } from '../interfaces/models';

const groupedPerDay = ref<{ date: Date, plants: PlantWithWateringsModel[], visibleCount: number }[]>([])

const handleWatered = (plant: PlantWithWateringsModel, date: Date) => {
    // Find index of row that threw the event
    const index = groupedPerDay.value.findIndex(r => isSameDay(r.date, date));

    // Find the plant id from the row and delete it
    groupedPerDay.value[index].visibleCount -= 1;

    // Insert a new element in the list at the date + dayInterval
    const newDate = addDays(groupedPerDay.value[index].date, plant.dayInterval);

    // Check if this new date exists
    const newIndex = groupedPerDay.value.findIndex(r => isSameDay(r.date, newDate));
    console.log(newIndex);
    if (newIndex !== -1) {
        groupedPerDay.value[newIndex].plants.push(plant);
        groupedPerDay.value[newIndex].visibleCount += 1;

        // Sort the grouped plants by date in ascending order
        groupedPerDay.value.sort((a, b) => a.date.getTime() - b.date.getTime());
    }
    else {
        // Then add the plant for the sliding effect
        groupedPerDay.value.push({ date: newDate, plants: [plant], visibleCount: 1 });
    }

    // Sort the grouped plants by date in ascending order
    groupedPerDay.value.sort((a, b) => a.date.getTime() - b.date.getTime());

    // we also have to add a watering
    api.addWatering(plant.id, date).catch(err => console.log('Couldn\'t add watering in the database', err));
}

onMounted(async () => {
    const plantsWithWatering = await api.getPlantsWithRecentWatering(new Date(), 40);
    const grouped = groupPlantPerNextWateringDay(plantsWithWatering);
    groupedPerDay.value = grouped.map(row => { return { ...row, visibleCount: row.plants.length } });
})
</script>

<style scoped>
.n-collapse-transition {
    .list-divider {
        width: 50%;
    }

    &:last-child .n-flex {
        .list-divider {
            display: none;
        }
    }
}
</style>