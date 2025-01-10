<template>
    <div class="calendar-item" :class="{
        'selected-item': selected,
        'current-day': currentDay,
        'other-month': isOtherMonth
    }" @click="onClick">
        <!-- Bottom bar when selected -->
        <div class="cell-bar" />

        <n-flex vertical style="height: 100%;">
            <n-flex align="center" justify="space-between">
                <div class="cell-day">{{ dateInfo.dayOfMonth }}</div>
                <div class="date-string desktop-only">{{ getDayName(dateInfo.date) }}</div>
            </n-flex>
            <div class="calendar-item-content">
                <n-skeleton v-if="loading" round size="small" />
                <n-badge v-else-if="plantsOnDay.length !== 0" :value="plantsOnDay.length - 1">
                    <n-avatar round size="small" :src="mainImage" object-fit="cover" />
                </n-badge>
            </div>
        </n-flex>
    </div>
</template>

<script setup lang="ts">
import { useThemeVars } from 'naive-ui'
import { onMounted, ref, watch } from 'vue';
import type { PlantWithWateringsModel } from "../../interfaces/models";
import { calcDayDifference } from '../../utils';

interface DateInfo {
    id: string;
    date: Date;
    dayOfMonth: number;
}

const props = defineProps<{
    dateInfo: DateInfo;
    selected: boolean;
    currentDay: boolean;
    isOtherMonth: boolean;
    plants: PlantWithWateringsModel[];
    loading: boolean
}>();

const emits = defineEmits(['clicked']);
const plantsOnDay = ref<PlantWithWateringsModel[]>([]);
const themeVars = useThemeVars();
const mainImage = ref<string | null>(null);

// Methods
const onClick = () => {
    emits('clicked', props.dateInfo.date);
};

const getFirstPlantImage = () => {
    const plantWithImage = plantsOnDay.value.find(p => p.image !== null);
    if (plantWithImage !== undefined) {
        return plantWithImage.image;
    }
    return null;
}

const getDayName = (date: Date): string => {
    return date.toLocaleDateString('default', { weekday: 'short' });
};

const mountFn = async (plants: PlantWithWateringsModel[], date: Date) => {
    // reset to be sure
    plantsOnDay.value = plants.filter(p =>
        p.waterings.some(w => calcDayDifference(w.dateWatered, date) === 0));
    mainImage.value = getFirstPlantImage();
}

onMounted(async () => await mountFn(props.plants, props.dateInfo.date));
watch([() => props.plants, () => props.dateInfo.date], async ([newPlants, newDate]) => {
    await mountFn(newPlants, newDate);
});
</script>

<style scoped>
.calendar-item {
    box-sizing: border-box;
    justify-content: center;
    align-items: center;
    border-right: 1px solid;
    border-bottom: 1px solid;
    padding: 10px;
    border-color: v-bind('themeVars.borderColor');
    cursor: pointer;
    transition: color .3s v-bind('themeVars.cubicBezierEaseIn'), border-color .3s v-bind('themeVars.cubicBezierEaseIn'), background-color .3s v-bind('themeVars.cubicBezierEaseIn');
    min-height: 100px;
    position: relative;

    &:hover {
        background-color: v-bind('themeVars.closeColorHover');
    }

    &:nth-child(7) {
        border-top-right-radius: v-bind('themeVars.borderRadius');
    }

    &:nth-child(1) {
        border-top-left-radius: v-bind('themeVars.borderRadius');
    }

    &:nth-child(29) {
        border-bottom-left-radius: v-bind('themeVars.borderRadius');
    }

    &:nth-child(35) {
        border-bottom-right-radius: v-bind('themeVars.borderRadius');
    }

    &:nth-child(-n+7) {
        border-top: 1px solid;
        border-color: v-bind('themeVars.borderColor');
    }

    &:nth-child(7n+1) {
        border-left: 1px solid;
        border-color: v-bind('themeVars.borderColor');
    }
}

.cell-bar {
    position: absolute;
    background-color: transparent;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 3px;
    z-index: 100;
    transition: .3s;
}

.selected-item .cell-bar {
    background-color: v-bind('themeVars.primaryColor');
}

.calendar-header-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
}

.cell-day {
    border-radius: 25%;
    width: 1.3em;
    height: 1.3em;
    display: flex;
    align-items: center;
    justify-content: center;
}

.current-day .cell-day {
    background-color: v-bind('themeVars.primaryColor');
    color: v-bind('themeVars.baseColor');
}

.other-month {
    color: v-bind('themeVars.textColorDisabled');
}

.date-string {
    font-size: 0.8rem;
}

.calendar-item-content {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>