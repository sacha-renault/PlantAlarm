<template>
    <n-flex vertical>
        <h1> Manage Plants </h1>
        <n-divider />

        <!-- Display current plant -->
        <!-- <n-card v-else v-for="plant in plants"> {{ plant.name }} {{ plant.waterQuantity }} </n-card> -->
        <plant-grid-manager :plants="plants" :n-skeleton="pageLoaded ? 0 : 6">
            <n-button style="height: 100%; width: 100%;" secondary type="success" @click="showModal = true"
                v-if="pageLoaded">
                <template #icon>
                    <AddIcon />
                </template>
                Add Plant
            </n-button>
        </plant-grid-manager>

        <!-- Add plant modal -->
        <add-plant-modal v-model="showModal" @plant-added="handleNewPlant" />
    </n-flex>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import AddPlantModal from '../modals/AddPlantModal.vue';
import { Add12Regular as AddIcon } from '@vicons/fluent'
import { PlantModel } from '../interfaces/models';
import { api } from '../api';
import { useMessage } from 'naive-ui';
import PlantGridManager from '../components/GridManage/PlantGridManager.vue';

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