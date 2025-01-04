<template>
    <n-layout>
        <!-- Header -->
        <n-layout-header bordered class="header">
            <m-header />
        </n-layout-header>

        <!-- Main Content Area -->
        <n-layout-content>
            <!-- FullChat with Fixed Height -->
            <div class="content-main-container">
                <RouterView />
            </div>
        </n-layout-content>
    </n-layout>
</template>

<script setup lang="ts">
import MHeader from '../components/MHeader.vue';
import { RouterView } from 'vue-router';
</script>

<style scoped>
.content-main-container {
    width: 100%;
    justify-content: center;
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>