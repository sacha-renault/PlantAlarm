<template>
    <div class="manage-plant-grid">
        <plant-card v-for="plant in plants" :plant="plant" />
        <card-skeleton v-for="_ in nSkeleton" />

        <div>
            <slot />
        </div>
    </div>
</template>

<script setup lang="ts">
import { PlantDto } from '../../interfaces/dto';
import PlantCard from './PlantCard.vue';
import CardSkeleton from './CardSkeleton.vue';

const { plants, nSkeleton } = defineProps<{ plants: PlantDto[], nSkeleton: number }>();
</script>

<style scoped>
.manage-plant-grid {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    padding: 1rem;
    gap: 0.5rem;

    >* {
        width: 100%;
        min-width: 0;
        min-height: 200px;
    }
}
</style>