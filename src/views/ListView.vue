<template>
    <div class="wf content-container">
        <h1>
            Plants to water
        </h1>
        <n-divider />
        <n-flex vertical class="wf" align="center">
            <h2> {{ formatDayString(selectedDay) }} </h2>
            <n-flex class="wf" style="gap: 0px;" v-if="plants.length !== 0">
                <plant-item-list v-for="plant in plants" :plant="plant" />
            </n-flex>
            <n-divider class="list-divider" />
        </n-flex>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { formatDayString, filterPlantsAtDay } from '../utils';
import { api } from '../api';
import PlantItemList from '../components/PlantItemList.vue';
import type { PlantWithWateringsModel } from '../interfaces/models';

const selectedDay = ref(new Date(Date.now()));
const plants = ref<PlantWithWateringsModel[]>([]);

onMounted(async () => {
    const plantsWithWatering = await api.getPlantsWithRecentWatering(new Date(), 40);
    console.log(plantsWithWatering);
    plants.value = filterPlantsAtDay(plantsWithWatering, selectedDay.value);
})
</script>

<style scoped>
.n-flex {
    .list-divider {
        width: 50%;
    }

    &:last-child .list-divider {
        display: none;
    }
}
</style>