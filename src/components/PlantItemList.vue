<template>
    <n-collapse-transition :show="show">
        <n-popover trigger="manual" :show="showPopover || showDate" placement="top">
            <template #trigger>
                <swipable-list-item @swipe-animation-over="handleSwipe" :animation-duration="0.45"
                    :replace-transition-duration="0.25" swipe-threshold="20%">
                    <n-flex align="center" justify="space-evenly" class="swipe-item-template">
                        <n-avatar round :src="plant.image" object-fit="cover" />
                        <n-divider vertical />
                        <n-space> {{ plant.name }} </n-space>
                        <n-divider vertical />
                        <n-space> {{ plant.waterQuantity }} mL </n-space>
                    </n-flex>
                    <template #icon-left>
                        <TimerIcon class="under-swip-icon-color" />
                    </template>
                    <template #icon-right>
                        <WaterIcon class="under-swip-icon-color" />
                    </template>
                </swipable-list-item>
            </template>

            <n-flex vertical v-if="showPopover">
                This plant shouldn't be watered today, confirm watering now ?

                <n-flex justify="space-evenly">
                    <n-button type="error" @click="showPopover = false"> Cancel </n-button>
                    <n-button type="success" @click="() => validate(date)"> Confirm</n-button>
                </n-flex>
            </n-flex>

            <n-flex vertical v-if="showDate">
                <h4>
                    Do you wanna add a custom water date ?
                </h4>

                <panel-datepicker v-model="customDate" :is-error="isDateError()" err-msg="You can't select a future date."/>

                <n-flex justify="space-evenly">
                    <n-button type="error" @click="showDate = false"> No </n-button>
                    <n-button type="success" @click="() => validate(new Date(customDate))"> Yes </n-button>
                </n-flex>
            </n-flex>

        </n-popover>
    </n-collapse-transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import type { PlantWithWateringsModel } from '../interfaces/models';
import SwipableListItem from './SwipableListItem.vue';
import { CalendarClock24Regular as TimerIcon, Drop20Regular as WaterIcon } from '@vicons/fluent'
import { useThemeVars } from 'naive-ui';
import { isSameDay } from '../utils';
import PanelDatepicker from '../components/PanelDatepicker.vue'

const themeVars = useThemeVars();
const showPopover = ref(false);
const showDate = ref(false);
const show = ref(true);
const now = ref(new Date());
const customDate = ref(new Date().getTime());
const { plant, date } = defineProps<{ plant: PlantWithWateringsModel, date: Date }>();
const emits = defineEmits(['plantWatered'])

const handleSwipe = (direction: string) => {
    if (direction === 'right') {
        if (date > now.value && !isSameDay(date, now.value)) {
            showPopover.value = true;
        } else {
            validate(date);
        }
    }

    else { // direction is 'left'
        showDate.value = true;
    }
}

const validate = (d: Date) => {
    setTimeout(() => show.value = false, 450);
    emits('plantWatered', plant, d);
}

const isDateError = () => {
    return !isSameDay(new Date(customDate.value), now.value) && customDate.value > now.value.getTime()
}
</script>

<style scoped lang="scss">

.swipe-item-template {
    padding: 0.5rem 1rem;
    border: 1px solid;
    border-radius: v-bind('themeVars.borderRadius');
    border-color: v-bind('themeVars.borderColor');
    background-color: v-bind('themeVars.bodyColor');
}

.under-swip-icon-color {
    color: v-bind('themeVars.baseColor')
}
</style>