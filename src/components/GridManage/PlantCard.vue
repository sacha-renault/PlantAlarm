<template>
    <n-card class="plant-card">
        <template #cover>
            <div class="image-container">
                <img :src="plant.image ?? ''" class="plant-image" />
            </div>
        </template>

        <template #header>
            <n-flex justify="space-between" align="center">
                <n-space>
                    {{ plant.name }}
                </n-space>
            </n-flex>
        </template>

        <n-flex>
            <n-space justify="space-between" align="center">
                <n-text depth="3">Water Interval</n-text>
                <n-text>{{ plant.dayInterval }} days</n-text>
            </n-space>

            <n-space justify="space-between" align="center">
                <n-text depth="3">Water Amount</n-text>
                <n-text>{{ plant.waterQuantity }}ml</n-text>
            </n-space>
        </n-flex>
    </n-card>
</template>

<script setup lang="ts">
import { PlantDto } from '../../interfaces/dto';

const { plant } = defineProps<{ plant: PlantDto }>();

</script>

<style scoped>
.plant-card:hover {
    transition: 0.3s;
    transform: translateY(-5px);
    cursor: pointer;
}

.image-container {
    overflow: hidden;
}

.plant-image {
    width: 100%;
    height: 100%;
    max-height: 100px;
    object-fit: cover;
}

.plant-name {
    font-size: 1.2em;
}
</style>