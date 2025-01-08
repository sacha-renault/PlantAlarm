<template>
    <n-collapse-transition :show="show">
        <swipable-list-item
            @swiped-left="handleSwipe('left')"
            @swiped-right="handleSwipe('right')"
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
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { FullPlantsDto } from '../api';
import SwipableListItem from './SwipableListItem.vue';
import { Timer16Regular as TimerIcon, Drop20Regular as WaterIcon } from '@vicons/fluent'
import { useThemeVars } from 'naive-ui';

const themeVars = useThemeVars();
const show = ref(true);
const { plant } = defineProps<{plant: FullPlantsDto}>();

const handleSwipe = (_: string) => {
    setTimeout(() => show.value = false, 450);
}
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