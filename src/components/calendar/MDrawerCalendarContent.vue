<template>
    <n-drawer to="#drawer-target" v-model:show="drawerShow" placement="top" height="50%">
        <n-drawer-content :title="'Plants to water (' + formatDateWithWeekday(date) + ')'">
            <!-- Data  -->
            <n-data-table :columns="columns" :data="plantsOnDay" :bordered="false" v-if="plantsOnDay.length !== 0" />

            <!-- In case no plant to water -->
            <n-empty v-else>
                No plant to water on that day
            </n-empty>

            <!-- Footer to close -->
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
import { NAvatar, NButton } from 'naive-ui';
import { filterPlantsAtDay } from '../../utils';
import { PlantWithWateringsModel } from '../../api';
import { onMounted, ref, watch, h } from 'vue';
import { formatDateWithWeekday } from '../../utils';

interface PlantDataDisplay {
    img: string
    name: string
    waterQuantity: number
}

const columns = ref([
    {
        title: 'Image', key: 'img', render(row: PlantDataDisplay) {
            return h(
                NAvatar, {
                size: 'medium',
                round: true,
                src: row.img
            })
        }
    },
    {
        title: 'Name', key: 'name'
    },
    {
        title: 'Water (mL)', key: 'waterQuantity'
    },
]);

const drawerShow = defineModel({ required: true, default: false });
const plantsOnDay = ref<PlantWithWateringsModel[]>([]);
const { date, plants } = defineProps<{ date: Date, plants: PlantWithWateringsModel[] }>();

onMounted(async () => {
    plantsOnDay.value = filterPlantsAtDay(plants, date);
})

watch(() => date, (newValue) => {
    plantsOnDay.value = filterPlantsAtDay(plants, newValue);
})

</script>