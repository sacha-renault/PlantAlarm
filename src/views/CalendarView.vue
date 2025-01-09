<template>
    <n-flex vertical style="height: 100%;">
        <h1>
            Water calendar
        </h1>
        <n-divider />
        <m-calendar id="drawer-target" @day-clicked="handleClick" @month-focus-changed="handleMonthChanged"
            :plants="plants" :loading="isLoading" />
        <m-drawer-calendar-content v-model="drawerShow" :date="date" :plants="plants" />
    </n-flex>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MCalendar from '../components/calendar/MCalendar.vue';
import MDrawerCalendarContent from '../components/calendar/MDrawerCalendarContent.vue';
import { api } from '../api';
import type { PlantWithWateringsModel } from '../interfaces/models';

const plants = ref<PlantWithWateringsModel[]>([]);
const drawerShow = ref(false);
const date = ref<Date>(new Date());
const isLoading = ref(true)
const handleClick = (d: Date) => {
    drawerShow.value = true;
    date.value = d;
}

const handleMonthChanged = (date: Date, numDays: number) => {
    console.log(date, numDays);
    isLoading.value = true;
    api.getPlantsWithRecentWatering(date, numDays).then(data => {
        plants.value = data;
    }).finally(() => isLoading.value = false)
}

onMounted(async () => {
    plants.value = await api.getPlantsWithRecentWatering(date.value, 35);
    console.log(plants.value);
    isLoading.value = false
})
</script>