<template>
    <n-flex vertical class="content-container">
        <h1> Manage Plants </h1>
        <n-divider />

        <!-- Button to add plant -->
        <n-button secondary type="success" @click="showModal = true" :disabled="!pageLoaded">
            <template #icon>
                <AddIcon />
            </template>
            Add Plant
        </n-button>

        <!-- Display current plant -->
        <n-skeleton :repeat="6" width="100%" :sharp="false" v-if="!pageLoaded"/>
        <n-card v-else v-for="plant in plants"> {{ plant.name }} {{ plant.waterQuantity }} </n-card>
    </n-flex>
    <add-plant-modal v-model="showModal" @plant-added="handleNewPlant" />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import AddPlantModal from '../modals/AddPlantModal.vue';
import { Add12Regular as AddIcon } from '@vicons/fluent'
import { PlantModel } from '../interfaces/models';
import { api } from '../api';
import { useMessage } from 'naive-ui';

const showModal = ref(false);
const plants = ref<PlantModel[]>([]);
const message = useMessage();
const pageLoaded = ref(false);

const handleNewPlant = (plant: PlantModel) => {
    plants.value.push(plant);
}

onMounted(() => {
    api.getAllPlants()
        .then(data => {
            plants.value.push(...data);
            pageLoaded.value = true;
        })
        .catch(err => message.error("Error fetching the plants: " + err));
});
</script>