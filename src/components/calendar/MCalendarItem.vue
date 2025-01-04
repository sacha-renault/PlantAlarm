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
                <div class="cell-day">{{ date.dayOfMonth }}</div>
                <div>{{ getDayName(date.date) }}</div>
            </n-flex>
            <div class="calendar-item-content">
                <n-badge value="999+">
                    <n-avatar size="medium" />
                </n-badge>
            </div>
        </n-flex>
    </div>
</template>

<script setup lang="ts">
import { useThemeVars } from 'naive-ui'

interface DateInfo {
    id: string;
    date: Date;
    dayOfMonth: number;
}

const props = defineProps<{
    date: DateInfo;
    selected: boolean;
    currentDay: boolean;
    isOtherMonth: boolean;
}>();

const emits = defineEmits(['clicked']);
const themeVars = useThemeVars();

const onClick = () => {
    emits('clicked', props.date.date);
};

const getDayName = (date: Date): string => {
    return date.toLocaleDateString('default', { weekday: 'short' });
};
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
    min-height: 125px;
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
    aspect-ratio: 1 / 1;
    height: 100%;
    border-radius: 50%;
    width: 1.8em;
    height: 1.8em;
    display: flex;
    align-items: center;
    justify-content: center;
}

.current-day .cell-day {
    background-color: v-bind('themeVars.primaryColor');
}

.other-month {
    color: v-bind('themeVars.textColorDisabled');
}

.calendar-item-content {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>