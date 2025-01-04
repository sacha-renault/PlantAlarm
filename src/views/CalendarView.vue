<template>
    <n-flex class="calendar-container">
        <m-calendar />
    </n-flex>
</template>

<script setup lang="ts">
import { addDays, isYesterday } from 'date-fns'
import { useMessage } from 'naive-ui'
import { defineComponent, ref } from 'vue'
import MCalendar from '../components/calendar/MCalendar.vue';

const msgProvider = useMessage();
const value = ref(null);
const handleUpdateValue = (_: number,
    { year, month, date }: { year: number, month: number, date: number }) => {
    msgProvider.success(`${year}-${month}-${date}`);
}
const isDateDisabled = (timestamp: number) => {
    if (isYesterday(timestamp)) {
        return true
    }
    return false
}
const test = `
<n-button> salut </n-button>
`
</script>

<style scoped>
.calendar-container {
    padding: 1rem;
    flex-grow: 1;
    max-width: 1024px;
    width: 100%;
}
</style>