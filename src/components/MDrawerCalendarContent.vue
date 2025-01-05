<template>
    <n-drawer to="#drawer-target" v-model:show="drawerShow" placement="top">
        <n-drawer-content :title="'Plants to water (' + formatDateWithWeekday(date) + ')'">
            <!-- Mocked data ... -->
            <n-list v-if="plantsOnDay.length !== 0">
                <n-list-item v-for="plant in plantsOnDay"> {{ plant.name }} </n-list-item>
            </n-list>

            <n-empty v-else>
                No plant to water on that day
            </n-empty>

            <template #footer>
                <n-button round @click="drawerShow = false" secondary type="error" size="tiny">
                    <template #icon>
                        <DismissIcon />
                    </template>
                </n-button>
            </template>
        </n-drawer-content>
    </n-drawer>

</template>
<script setup lang="ts">
import {
    Dismiss28Filled as DismissIcon,
} from '@vicons/fluent'
import { filterPlantsAtDay } from '../utils';
import { FullPlantsDto } from '../api';
import { onMounted, ref, watch } from 'vue';

const drawerShow = defineModel({ required: true, default: false });
const plantsOnDay = ref<FullPlantsDto[]>([]);
const { date, plants } = defineProps<{ date: Date, plants: FullPlantsDto[] }>();
const formatDateWithWeekday = (date: Date | any): string => {
    const options: Intl.DateTimeFormatOptions = {
        weekday: 'long',  // Full name of the weekday
        day: 'numeric',   // Day of the month
        month: 'long',    // Full name of the month
    };
    return date?.toLocaleDateString('en-US', options);
};

onMounted(async () => {
    plantsOnDay.value = filterPlantsAtDay(plants, date);
});

watch(() => date, (newValue) => {
    plantsOnDay.value = filterPlantsAtDay(plants, newValue);
})

</script>