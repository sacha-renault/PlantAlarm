<template>
    <n-layout>
        <n-layout-header>
            <n-flex align="center" justify="space-between">
                <h3>
                    {{ currentMonthDisplay }}
                </h3>
                <n-popover trigger="click" placement="left">
                    <template #trigger>
                        <n-button>
                            <template #icon>
                                <FIlterIcon />
                            </template>
                        </n-button>
                    </template>

                    <!-- Content of filter -->
                    <n-flex vertical style="padding: 1rem;">
                        <h4>Choose plant(s) to display</h4>
                        <n-divider />
                        <n-checkbox :checked="numberSelected === plantSelected.length"
                            :indeterminate="!(numberSelected === 0 || numberSelected === plantSelected.length)">
                            All plants
                        </n-checkbox>
                        <n-checkbox v-for="(plant, index) in plants" :key="index"
                            v-model:checked="plantSelected[index]">
                            {{ plant.name }}
                        </n-checkbox>
                    </n-flex>
                </n-popover>
            </n-flex>
        </n-layout-header>
        <n-layout>
            <div class="calendar-grid-layout">
                <m-calendar-item v-for="date in calendarDays" :key="date.id" :date="date"
                    :selected="selectedDate !== null && isSameDay(selectedDate, date.date)"
                    :current-day="isSameDay(today, date.date)" :is-other-month="!isSameMonth(currentMonth, date.date)"
                    @clicked="handleClick" :plants="plants" />
            </div>
        </n-layout>
        <n-layout-footer>
            <n-flex>
                <n-button-group class="wf">
                    <n-button @click="previousMonth">
                        <template #icon>
                            <BackwardIcon />
                        </template>
                    </n-button>
                    <n-button @click="goToToday" style="flex-grow: 1;">today</n-button>
                    <n-button @click="nextMonth">
                        <template #icon>
                            <ForwardIcon />
                        </template>
                    </n-button>
                </n-button-group>
            </n-flex>
        </n-layout-footer>
    </n-layout>
</template>

<script setup lang="ts">
import { ArrowRight28Filled as ForwardIcon, ArrowLeft28Filled as BackwardIcon, Filter16Filled as FIlterIcon } from '@vicons/fluent'
import MCalendarItem from './MCalendarItem.vue';
import { ref, computed } from 'vue';
import type { FullPlantsDto } from '../../api';

const today = new Date();
const currentMonth = ref(new Date());
const selectedDate = ref<Date | null>(null);
const { plants } = defineProps<{ plants: FullPlantsDto[] }>()
const plantSelected = ref(plants.map(_ => true));
const emits = defineEmits(['dayClicked']);

// Format the current month display
const currentMonthDisplay = computed(() => {
    return currentMonth.value.toLocaleDateString('default', {
        month: 'long',
        year: 'numeric'
    });
});

// Get all days for the calendar
const calendarDays = computed(() => {
    const year = currentMonth.value.getFullYear();
    const month = currentMonth.value.getMonth();

    // Get first day of month
    const firstDay = new Date(year, month, 1);
    // Get last day of month
    const lastDay = new Date(year, month + 1, 0);

    // Get the day of week for the first day (0 = Sunday)
    const firstDayOfWeek = firstDay.getDay();

    // Calculate days needed from previous month
    const prevMonthDays = [];
    const prevMonthLastDay = new Date(year, month, 0).getDate();
    for (let i = firstDayOfWeek - 1; i >= 0; i--) {
        prevMonthDays.push({
            id: `prev-${i}`,
            date: new Date(year, month - 1, prevMonthLastDay - i),
            dayOfMonth: prevMonthLastDay - i
        });
    }

    // Current month days
    const currentMonthDays = [];
    for (let i = 1; i <= lastDay.getDate(); i++) {
        currentMonthDays.push({
            id: `current-${i}`,
            date: new Date(year, month, i),
            dayOfMonth: i
        });
    }

    // Calculate how many days needed from next month
    const totalDays = 35; // 5 rows × 7 days
    const nextMonthDays = [];
    const remainingDays = totalDays - (prevMonthDays.length + currentMonthDays.length);
    for (let i = 1; i <= remainingDays; i++) {
        nextMonthDays.push({
            id: `next-${i}`,
            date: new Date(year, month + 1, i),
            dayOfMonth: i
        });
    }

    return [...prevMonthDays, ...currentMonthDays, ...nextMonthDays];
});

const handleClick = (date: Date) => {
    selectedDate.value = date;
    emits('dayClicked', date);
};

const nextMonth = () => {
    currentMonth.value = new Date(
        currentMonth.value.getFullYear(),
        currentMonth.value.getMonth() + 1,
        1
    );
};

const previousMonth = () => {
    currentMonth.value = new Date(
        currentMonth.value.getFullYear(),
        currentMonth.value.getMonth() - 1,
        1
    );
};

const goToToday = () => {
    currentMonth.value = new Date();
    selectedDate.value = new Date();
};

// Utility functions
const isSameDay = (date1: Date, date2: Date): boolean => {
    return date1.getDate() === date2.getDate() &&
        date1.getMonth() === date2.getMonth() &&
        date1.getFullYear() === date2.getFullYear();
};

const isSameMonth = (date1: Date, date2: Date): boolean => {
    return date1.getMonth() === date2.getMonth() &&
        date1.getFullYear() === date2.getFullYear();
};

const numberSelected = computed(() => plantSelected.value.filter(item => item === true).length)
</script>

<style scoped>
.calendar-grid-layout {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    grid-template-rows: repeat(5, 1fr);
}
</style>