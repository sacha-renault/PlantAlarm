<template>
    <!-- Button in center -->
    <n-flex style="grid-area: nav" justify="space-evenly" align="center">
        <n-button @click="handleMenuClick('calendar')" :bordered="false" ghost
            :class="{ 'nav-button-active': isButtonActive('calendar') }">
            <template #icon>
                <CalendarIcon />
            </template>
            <div class="desktop-only">
                Calendar
            </div>
        </n-button>
        <n-button @click="handleMenuClick('list')" :bordered="false" ghost
            :class="{ 'nav-button-active': isButtonActive('list') || isButtonActive('') }"
            :color="buttonColor('list') ?? buttonColor('')">
            <template #icon>
                <ListIcon />
            </template>
            <div class="desktop-only">
                List
            </div>
        </n-button>
        <n-button @click="handleMenuClick('plants')" :bordered="false" ghost
            :class="{ 'nav-button-active': isButtonActive('plants') }">
            <template #icon>
                <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
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
    </n-flex>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import {
    CalendarClock24Regular as CalendarIcon,
    ClipboardBulletListRtl20Regular as ListIcon
} from '@vicons/fluent';
import { useThemeVars } from 'naive-ui';
import { router } from '../routers/main.ts'

const themeVars = useThemeVars();
const selectedButton = ref('calendar');

const handleMenuClick = (buttonName: string) => {
    selectedButton.value = buttonName;
    router.push('/' + buttonName);
}
const isButtonActive = (buttonName: string) => {
    const routerVal = router.currentRoute.value.path.substring(1);
    return buttonName === routerVal;
}

const buttonColor = (buttonName: string): string => {
    return isButtonActive(buttonName) ? themeVars.value.primaryColor : '';
}
</script>

<style scoped>
.n-button {
    &::after {
        transition: 0.5s ease;
        content: "";
        background-color: v-bind('themeVars.primaryColor');
        border-radius: v-bind('themeVars.borderRadius');
        position: absolute;
        left: 0;
        bottom: 0;
        width: 0%;
        height: 7%;
    }

    &:hover {
        &::after {
            width: 100%;
        }
    }
}

.nav-button-active {
    &::after {
        width: 100%;
    }
}
</style>