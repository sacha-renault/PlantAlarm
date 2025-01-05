<template>
    <n-flex class="wf main-swipable-container">
        <!-- Main display -->
        <n-flex align="center" justify="space-evenly" @touchstart="handleTouchStart" @touchmove="handleTouchMove"
            @touchend="handleTouchEnd" :style="{ transform: 'translateX(' + currentX.toString() + 'px)' }"
            class="wf swipable-container">
            <n-avatar round />
            <n-divider vertical />
            <n-space> name </n-space>
            <n-divider vertical />
            <n-space> water qty </n-space>
        </n-flex>

        <div class="under-swipe-container">
            <n-button class="under-swipe-part" primary type="warning">
                <TimerIcon class="icon-small icon-grow" :class="[{ 'swiped': isSwipedLeft }]" />
            </n-button>
            <n-button class="under-swipe-part" primary type="info">
                <WaterIcon class="icon-small icon-grow" :class="[{ 'swiped': isSwipedRight }]" />
            </n-button>
        </div>
    </n-flex>
</template>

<script setup lang="ts">
import { useThemeVars } from 'naive-ui';
import { Timer16Regular as TimerIcon, Drop20Regular as WaterIcon } from '@vicons/fluent'
import { ref } from 'vue';

const themeVars = useThemeVars();

// min and max heigh of icone under the main structure
const requiredDrag = 150;
const animationTime = 0.5;

const isSwipedRight = ref(false);
const isSwipedLeft = ref(false);
const absX = ref(0);
const startX = ref(0);
const currentX = ref(0);
const isDragging = ref(false);
const emits = defineEmits(['swipedLeft', 'swipedRight'])

const handleTouchStart = (e: TouchEvent) => {
    startX.value = e.touches[0].clientX;
    isDragging.value = true;
};

const handleTouchMove = (e: TouchEvent) => {
    if (!isDragging.value) return;
    const deltaX = e.touches[0].clientX - startX.value;
    if (Math.abs(deltaX) <= requiredDrag) {
        currentX.value = deltaX;
        absX.value = Math.abs(deltaX);
    }
};

const handleTouchEnd = () => {
    isDragging.value = false;

    if (Math.abs(currentX.value) < 75) {
        currentX.value = 0;
        return;
    }

    // Trigger animation based on direction
    if (currentX.value < 0) {
        isSwipedRight.value = true;
        emits('swipedRight');
    } else {
        isSwipedLeft.value = true;
        emits('swipedLeft');
    }

    // Reset after animation
    setTimeout(() => {
        currentX.value = 0;
        isSwipedLeft.value = false;
        isSwipedRight.value = false;
    }, animationTime * 1000); // Match animation duration
};

</script>

<style scoped>
.main-swipable-container {
    position: relative;
    width: 100%;
    height: 100%;
}

.swipable-container {
    box-sizing: border-box;
    padding: 0.5rem 1rem;
    border: 1px solid;
    border-radius: v-bind('themeVars.borderRadius');
    border-color: v-bind('themeVars.borderColor');
    background-color: v-bind('themeVars.bodyColor');
    z-index: 100;
}

.under-swipe-container {
    display: flex;
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: v-bind('themeVars.borderRadius');
    z-index: 0;
    flex-flow: nowrap;
    margin: 0;
    padding: 0;
}

.under-swipe-part {
    box-sizing: border-box;
    width: 50%;
    height: 100%;
    border-radius: v-bind('themeVars.borderRadius');
    flex-shrink: 0;
    display: flex;
    align-items: center;
    padding: 1rem;
    overflow: hidden;

    &:first-child {
        justify-content: start;
    }

    &:last-child {
        justify-content: end;
    }
}

.icon-grow {
    --base-scale: calc(1 + v-bind(absX) / 200);
    transform: scale(var(--base-scale));
}

.swiped {
    animation: growAndSpin .5s ease infinite;
}

@keyframes growAndSpin {
    0% {
        transform: scale(var(--base-scale)) rotate(0deg);
    }

    50% {
        transform: scale(calc(var(--base-scale) * 0.75));
    }

    100% {
        transform: scale(var(--base-scale));
    }
}
</style>