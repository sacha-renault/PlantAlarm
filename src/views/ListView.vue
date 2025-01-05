<template>
    <div class="view-list-container wf">
        <h1>
            Plants to water
        </h1>
        <n-divider />
        <n-flex vertical class="wf" align="center">
            <h2> {{ displayDay() }} </h2>
            <n-flex class="wf" style="gap: 0px;" v-if="plants.length !== 0">
                <swipable-list-item v-for="plant in plants" :name="plant.name" :water-qty="plant.waterQuantity"
                    :img="plant.img ?? ''" @swiped-left="console.log('swiped left')" />
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
const onDateChanged = async () => {
    // this force a reset of all SwipableListItem
    plants.value = [];

    // then we assign new values
    plants.value = filterPlantsAtDay(await api.getAllPlants(), selectedDay.value);
}

onMounted(async () => {
    plants.value = filterPlantsAtDay(await api.getAllPlants(), selectedDay.value);
})
</script>

<style scoped>
h1 {
    font-size: clamp(2rem, calc(5vw + 1rem), 5rem);
}

.view-list-container {
    width: min(80%, 1024px);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    padding: 0.5rem;
}

.bottom-paginator {
    margin: auto 12rem;
}
</style>