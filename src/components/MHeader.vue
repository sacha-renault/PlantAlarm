<template>
    <n-collapse-transition :show="show" class="collapse-container">
        <n-flex class="header-container" align="center" justify="space-between">
            <!-- Main Logo + Drawer-->
            <n-flex align="center">
                <plant-alarm-logo class="icon-medium" />
                <div class="title"> PlantAlarm </div>
            </n-flex>

            <!-- Button in center -->
            <n-button-group class="nav-buttons" size="large">
                <n-button @click="handleMenuClick('calendar')" :bordered="false" :color="buttonColor('calendar')" ghost>
                    <template #icon>
                        <CalendarIcon />
                    </template>
                    <div class="desktop-only">
                        Calendar
                    </div>
                </n-button>
                <n-button @click="handleMenuClick('list')" :bordered="false" :color="buttonColor('list')" ghost>
                    <template #icon>
                        <ListIcon />
                    </template>
                    <div class="desktop-only">
                        List
                    </div>
                </n-button>
                <n-button @click="handleMenuClick('plants')" :bordered="false" :color="buttonColor('plants')" ghost>
                    <template #icon>
                        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
                            viewBox="0 0 24 24">
                            <g fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round">
                                <path d="M7 15h10v4a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2v-4z"></path>
                                <path d="M12 9a6 6 0 0 0-6-6H3v2a6 6 0 0 0 6 6h3"></path>
                                <path d="M12 11a6 6 0 0 1 6-6h3v1a6 6 0 0 1-6 6h-3"></path>
                                <path d="M12 15V9"></path>
                            </g>
                        </svg>
                    </template>
                    <div class="desktop-only">
                        Manage Plants
                    </div>
                </n-button>
            </n-button-group>

            <!-- Right layout -->
            <n-flex>
                <n-switch size="large" v-model:value="themeStore.isDark">
                    <template #checked-icon>
                        ☀️
                    </template>
                    <template #unchecked-icon>
                        🌙
                    </template>
                </n-switch>
            </n-flex>
        </n-flex>
    </n-collapse-transition>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useThemeStore } from '../stores/theme';
import {
    CalendarClock24Regular as CalendarIcon,
    ClipboardBulletListRtl20Regular as ListIcon
} from '@vicons/fluent';
import { useThemeVars } from 'naive-ui';
import PlantAlarmLogo from './PlantAlarmLogo.vue';
import { router } from '../routers/main.ts'

const show = ref(true);
const themeStore = useThemeStore();
const selectedButton = ref('calendar');

const handleMenuClick = (buttonName: string) => {
    selectedButton.value = buttonName;
    router.push('/' + buttonName);
}
const buttonColor = (buttonName: string) => {
    return selectedButton.value === buttonName ? useThemeVars().value.primaryColor : '';
}
</script>

<style scoped>
.title {
    font-weight: bold;
    font-size: 110%;
}

.header-container {
    padding: 2rem 3rem;
}

.nav-buttons {
    justify-content: space-evenly;
}

@media (max-width: 768px) {
    .nav-buttons {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 12px;
        background-color: var(--n-color);
        box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
        z-index: 1000;

        >* {
            width: 30%;
        }
    }
}
</style>