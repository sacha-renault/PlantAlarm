<template>
    <div class="view-list-container">
        <h1>
            Plants to water
        </h1>
        <n-divider />
        <n-flex vertical class="wf" align="center">
            <h2> {{ displayDay() }} </h2>
            <n-flex class="wf" style="gap: 0px;">
                <SwipableListItem />
                <SwipableListItem />
                <SwipableListItem />
                <SwipableListItem />
            </n-flex>
            <day-paginator v-model="selectedDay" @date-changed="console.log(selectedDay)" />
        </n-flex>
    </div>

</template>

<script setup lang="ts">
import { ref } from 'vue';
import DayPaginator from '../components/DayPaginator.vue';
import SwipableListItem from '../components/SwipableListItem.vue';
import { calcDayDifference } from '../utils';

const selectedDay = ref(new Date(Date.now()));
const today = ref(new Date(Date.now()));

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
</style>