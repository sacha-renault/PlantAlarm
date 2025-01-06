<template>
    <n-flex vertical class="content-container">
        <h1> Manage Plants </h1>
        <n-divider />

        <!-- Button to add plant -->
        <n-button secondary type="success" @click="showModal = true">
            <template #icon>
                <AddIcon />
            </template>
            Add Plant
        </n-button>
    </n-flex>
    <add-plant-modal v-model="showModal" />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import AddPlantModal from '../modals/AddPlantModal.vue';
import { Add12Regular as AddIcon } from '@vicons/fluent'

const showModal = ref(false);

</script>