<template>
    <n-drawer to="#drawer-target" v-model:show="drawerShow" placement="top">
        <n-drawer-content :title="'Plants to water (' + formatDateWithWeekday(date) + ')'">
            <!-- Mocked data ... -->
            <n-list>
                <n-list-item> a </n-list-item>
                <n-list-item> a </n-list-item>
                <n-list-item> a </n-list-item>
                <n-list-item> a </n-list-item>
            </n-list>

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

const drawerShow = defineModel({ required: true, default: false });
const { date } = defineProps({
    date: Date
})
const formatDateWithWeekday = (date: Date | any): string => {
    const options: Intl.DateTimeFormatOptions = {
        weekday: 'long',  // Full name of the weekday
        day: 'numeric',   // Day of the month
        month: 'long',    // Full name of the month
    };
    return date?.toLocaleDateString('default', options);
};

</script>