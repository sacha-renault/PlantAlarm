<template>
    <div v-if="isLoading">Loading...</div>
    <n-flex class="calendar-container content-container" v-else>
        <m-calendar id="drawer-target" @day-clicked="handleClick" :plants="plants" />
        <m-drawer-calendar-content v-model="drawerShow" :date="date" :plants="plants" />
    </n-flex>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MCalendar from '../components/calendar/MCalendar.vue';
import MDrawerCalendarContent from '../components/calendar/MDrawerCalendarContent.vue';
import type { PlantWithWateringsModel } from '../api';
import { api } from '../api';

const plants = ref<PlantWithWateringsModel[]>([]);
const drawerShow = ref(false);
const date = ref<Date>(new Date());
const isLoading = ref(true)
const handleClick = (d: Date) => {
    // drawerShow.value = true;
    date.value = d;
}

onMounted(async () => {
    plants.value = await api.getPlantsWithRecentWatering()
    isLoading.value = false
})
</script>