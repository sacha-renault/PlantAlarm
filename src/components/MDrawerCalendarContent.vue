<template>
    <n-drawer to="#drawer-target" v-model:show="drawerShow" placement="top">
        <n-drawer-content :title="'Plants to water (' + date?.toLocaleDateString('default', { weekday: 'long' }) + ')'">
            Stoner is a 1965 novel by the American writer John Williams.

            <template #footer>
                <n-flex>
                    <n-button round @click="drawerShow = false" secondary type="error">
                        <template #icon>
                            <DismissIcon />
                        </template>
                    </n-button>
                </n-flex>
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

</script>