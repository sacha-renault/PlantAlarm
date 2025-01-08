<template>
    <div class="wf content-container">
        <h1>
            Plants to water
        </h1>
        <n-divider />
        <n-flex vertical class="wf" align="center">
            <h2> {{ displayDay() }} </h2>
            <n-flex class="wf" style="gap: 0px;" v-if="plants.length !== 0">
                <swipable-list-item v-for="plant in plants" @swiped-left="console.log('swiped left')" :animation-duration="1">
                    <n-avatar round :src="plant.img" />
                    <n-divider vertical />
                    <n-space> {{ plant.name }} </n-space>
                    <n-divider vertical />
                    <n-space> {{ plant.waterQuantity }} mL </n-space>

                    <template #icon-left>
                        <TimerIcon style="color: black"/>
                    </template>
                    <template #icon-right>
                        <WaterIcon style="color: black"/>
                    </template>
                </swipable-list-item>
            </n-flex>
            <n-divider style="width:50%" />
        </n-flex>
    </div>

</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import SwipableListItem from '../components/SwipableListItem.vue';
import { calcDayDifference, filterPlantsAtDay } from '../utils';
import { api, FullPlantsDto } from '../api';
import { Timer16Regular as TimerIcon, Drop20Regular as WaterIcon } from '@vicons/fluent'

const selectedDay = ref(new Date(Date.now()));
const today = ref(new Date(Date.now()));
const plants = ref<FullPlantsDto[]>([]);

// methods
const displayDay = () => {
    const dayDiff = calcDayDifference(selectedDay.value, today.value);
    if (dayDiff === 0) {
        return 'Today';
    } else if (selectedDay.value.getDate() < today.value.getDate()) {
        if (dayDiff === 1) {
            return 'Yesterday';
        } else {
            return dayDiff.toString() + ' days ago';
        }
    } else {
        if (dayDiff === 1) {
            return 'Tomorrow';
        } else {
            return 'In ' + dayDiff.toString() + ' days';
        }
    }
}

onMounted(async () => {
    plants.value = filterPlantsAtDay(await api.getPlantsWithRecentWatering(), selectedDay.value);
})
</script>

<style scoped>
.bottom-paginator {
    margin: auto 12rem;
}
</style>