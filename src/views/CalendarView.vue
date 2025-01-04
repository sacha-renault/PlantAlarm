<template>
    <n-flex class="calendar-container">
        <m-calendar id="drawer-target" @day-clicked="handleClick" />
        <m-drawer-calendar-content v-model="drawerShow" :date="date" />
    </n-flex>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import MCalendar from '../components/calendar/MCalendar.vue';
import MDrawerCalendarContent from '../components/MDrawerCalendarContent.vue';

const drawerShow = ref(false);
const date = ref<Date>(new Date());
const handleClick = (d: Date) => {
    drawerShow.value = true;
    date.value = d;
}
</script>

<style scoped>
.calendar-container {
    padding: 1rem;
    flex-grow: 1;
    max-width: 1024px;
    width: 90%;
}
</style>