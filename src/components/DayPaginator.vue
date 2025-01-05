<template>
    <n-flex align="center" justify="space-evenly" class="wf">
        <n-button-group>
            <n-button size="small" @click="prevDay">
                <template #icon>
                    <BackwardIcon />
                </template>
            </n-button>
            <n-button size="small" @click="toToday">
                Go to today
            </n-button>
            <n-button size="small" @click="nextDay">
                <template #icon>
                    <ForwardIcon />
                </template>
            </n-button>
        </n-button-group>
    </n-flex>
</template>

<script setup lang="ts">
import { ArrowRight28Filled as ForwardIcon, ArrowLeft28Filled as BackwardIcon } from '@vicons/fluent'
import { addDays } from '../utils';

const model = defineModel({ required: true, default: new Date(Date.now()) });
const emits = defineEmits(['dateChanged'])

// methods
const nextDay = () => {
    model.value = addDays(model.value, 1);
    emits('dateChanged');
}
const prevDay = () => {
    model.value = addDays(model.value, -1);
    emits('dateChanged');
}
const toToday = () => {
    model.value = new Date(Date.now());
    emits('dateChanged');
}

</script>