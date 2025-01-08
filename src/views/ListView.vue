<template>
    <div class="wf content-container">
        <h1>
            Plants to water
        </h1>
        <n-divider />
        <n-flex vertical class="wf" align="center">
            <h2> {{ displayDay() }} </h2>
            <n-flex class="wf" style="gap: 0px;" v-if="plants.length !== 0">
                <n-collapse-transition v-for="plant, index in plants" :show="showItem[index]">
                    <swipable-list-item
                        @swiped-left="handleSwipe(index, 'left')"
                        @swiped-right="handleSwipe(index, 'left')"
                        @swipe-animation-over="console.log('animation over')"
                        :animation-duration="0.45"
                        :replace-transition-duration="0.25"
                        swipe-threshold="20%">

                        <n-flex align="center" justify="space-evenly" class="swipe-item-template">
                            <n-avatar round :src="plant.img" />
                            <n-divider vertical />
                            <n-space> {{ plant.name }} </n-space>
                            <n-divider vertical />
                            <n-space> {{ plant.waterQuantity }} mL </n-space>
                        </n-flex>

                        <template #icon-left>
                            <TimerIcon style="color: black"/>
                        </template>
                        <template #icon-right>
                            <WaterIcon style="color: black"/>
                        </template>
                    </swipable-list-item>
                </n-collapse-transition>
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
import { useThemeVars } from 'naive-ui';

const themeVars = useThemeVars();
const selectedDay = ref(new Date(Date.now()));
const today = ref(new Date(Date.now()));
const plants = ref<FullPlantsDto[]>([]);
const showItem = ref<boolean[]>([])

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

const handleSwipe = (index: number, _: string) => {
    setTimeout(() => showItem.value[index] = false, 450);
}

onMounted(async () => {
    plants.value = filterPlantsAtDay(await api.getPlantsWithRecentWatering(), selectedDay.value);
    showItem.value = new Array(plants.value.length).fill(true);
})
</script>

<style scoped>
.swipe-item-template {
    padding: 0.5rem 1rem;
    border: 1px solid;
    border-radius: v-bind('themeVars.borderRadius');
    border-color: v-bind('themeVars.borderColor');
    background-color: v-bind('themeVars.bodyColor');
}


</style>