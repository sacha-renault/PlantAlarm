<template>
    <!-- Right layout -->
    <n-flex align="center" justify="center">
        <n-switch size="large" v-model:value="themeStore.isDark">
            <template #checked-icon>
                ☀️
            </template>
            <template #unchecked-icon>
                🌙
            </template>
        </n-switch>
    </n-flex>
</template>

<script setup lang="ts">
import { useThemeStore } from '../stores/theme';
const themeStore = useThemeStore();
</script>