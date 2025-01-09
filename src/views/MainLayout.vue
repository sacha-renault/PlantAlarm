<template>
    <div class="grid-main-layout">
        <!-- Header -->
        <PlantAlarmLogo class="header-style" style="grid-area: logo;" />
        <m-header class="header-style" style="grid-area: nav;" />
        <MOptions class="header-style" style="grid-area: options" />

        <div class="grid-main content-main-container">
            <div class="scroll-content-container">
                <RouterView />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import MOptions from '../components/MOptions.vue';
import MHeader from '../components/MHeader.vue';
import PlantAlarmLogo from '../components/PlantAlarmLogo.vue';
import { RouterView } from 'vue-router';
import { useThemeVars } from 'naive-ui';

const themeVars = useThemeVars();
</script>

<style scoped>
.content-main-container {
    width: 100%;
    height: 100%;
    justify-content: start;
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow-x: hidden;
    overflow-y: hidden;
    flex-grow: 1;
}

.scroll-content-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    overflow-y: auto;
    overflow-x: hidden;
    width: 100%;
    height: 100%;

    >* {
        box-sizing: border-box;
        width: min(80%, 1024px);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: start;
        overflow-x: hidden;
        flex-shrink: 0;
        padding: 1rem 0;
    }
}

.grid-main-layout {
    gap: 0;
    width: 100vw;
    height: 100dvh;
    display: grid;
    grid-template-columns: 1fr 4fr 1fr;
    grid-template-rows: clamp(50px, 10%, 75px) 1fr;
    overflow-x: hidden;
    overflow-y: hidden;

    grid-template-areas:
        "logo nav options"
        "main main main";

    .grid-main {
        grid-area: main;
    }

    .m-header {
        grid-area: nav;
    }
}

.header-style {
    /* padding: 2rem 3rem; */
    background-color: v-bind('themeVars.tabColor');
}

@media (max-width: 768px) {
    .grid-main-layout {
        grid-template-columns: 1fr 1fr;
        grid-template-rows: clamp(50px, 10%, 75px) 1fr clamp(50px, 10%, 75px);
        grid-template-areas:
            "logo options"
            "main main"
            "nav nav";
    }
}
</style>