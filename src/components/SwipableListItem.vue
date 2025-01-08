<template>
    <n-collapse-transition :show="show">
        <n-flex class="wf main-swipable-container">
            <!-- Main display -->
            <n-flex align="center" justify="space-evenly" @touchstart="touchHandlers.onTouchStart"
                @touchmove="touchHandlers.onTouchMove" @touchend="touchHandlers.onTouchEnd"
                @mousedown="mouseHandlers.onMouseDown" @mousemove="mouseHandlers.onMouseMove"
                @mouseup="mouseHandlers.onMouseUp" :style="{ transform: 'translateX(' + currentX.toString() + 'px)' }"
                class="wf swipable-container">
                <n-avatar round :src="img" />
                <n-divider vertical />
                <n-space> {{ name }} </n-space>
                <n-divider vertical />
                <n-space> {{ waterQty }} mL </n-space>
            </n-flex>
            <div class="under-swipe-container">
                <div class="under-swipe-part" :class="[{ 'swiping': isSwipingLeft }]">
                    <TimerIcon class="icon-small icon-grow"
                        :class="[{ 'swiped': isSwipedLeft }]" />
                </div>
                <div :class="[{ 'swiping': isSwipingRight, 'under-swipe-part': true }]">
                    <WaterIcon class="icon-small icon-grow" :class="[{ 'swiped': isSwipedRight }]" />
                </div>
            </div>
        </n-flex>
    </n-collapse-transition>
</template>

<script setup lang="ts">
import { useMessage, useThemeVars } from 'naive-ui';
import { Timer16Regular as TimerIcon, Drop20Regular as WaterIcon } from '@vicons/fluent'
import { ref, computed } from 'vue';

const themeVars = useThemeVars();
const message = useMessage();

// min and max heigh of icone under the main structure
const animationTime = 0.5;
const threshold = 75;
const maxSwipe = 150;
const leftColor = ref(themeVars.value.warningColor);
const rightColor = ref(themeVars.value.infoColor);

// vue ref
const absX = ref(0);
const startX = ref(0);
const currentX = ref(0);
const isDragging = ref(false);
const emits = defineEmits(['swipedLeft', 'swipedRight'])
const show = ref(true);
const isSwipedRight = ref(false);
const isSwipedLeft = ref(false);
const isSwipingRight = computed(() => currentX.value < 0);
const isSwipingLeft = computed(() => currentX.value > 0);

// see props
const { name, waterQty, img } = defineProps<{ name: string, waterQty: number, img: string }>()


// Swip methods
const handleDragStart = (x: number) => {
    startX.value = x;
    isDragging.value = true;
};

const handleDragMove = (x: number) => {
    if (!isDragging.value) return;

    const deltaX = x - startX.value;
    const absDeltaX = Math.abs(deltaX);

    if (maxSwipe > absDeltaX) {
        absX.value = absDeltaX;
        currentX.value = deltaX;
    }
};

const handleDragEnd = () => {
    isDragging.value = false;

    if (Math.abs(currentX.value) < threshold) {
        currentX.value = 0;
        return;
    }

    if (currentX.value < 0) {
        isSwipedRight.value = true;
        emits('swipedRight');
        message.success('right')
    } else {
        isSwipedLeft.value = true;
        emits('swipedLeft');
        message.success('left')
    }

    setTimeout(() => {
        currentX.value = 0;
        isSwipedLeft.value = false;
        isSwipedRight.value = false;
        show.value = false;
    }, animationTime * 1000);
};

// Event handlers that use the core drag logic
const touchHandlers = {
    onTouchStart: (e: TouchEvent) => handleDragStart(e.touches[0].clientX),
    onTouchMove: (e: TouchEvent) => handleDragMove(e.touches[0].clientX),
    onTouchEnd: () => handleDragEnd()
};

const mouseHandlers = {
    onMouseDown: (e: MouseEvent) => handleDragStart(e.clientX),
    onMouseMove: (e: MouseEvent) => handleDragMove(e.clientX),
    onMouseUp: () => handleDragEnd(),
};

</script>

<style scoped>
.main-swipable-container {
    position: relative;
    width: 100%;
    height: 100%;
    cursor: grab;
}

.swipable-container {
    padding: 0.5rem 1rem;
    border: 1px solid;
    border-radius: v-bind('themeVars.borderRadius');
    border-color: v-bind('themeVars.borderColor');
    background-color: v-bind('themeVars.bodyColor');
    z-index: 100;
}

.under-swipe-container {
    box-sizing: border-box;
    display: flex;
    position: absolute;
    width: 100%;
    height: 100%;
    border: none;
    border-radius: v-bind('themeVars.borderRadius');
    z-index: 0;
    flex-flow: nowrap;
    margin: 0;
    padding: 0;
    overflow: hidden;
}

.under-swipe-part {
    box-sizing: border-box;
    border-radius: v-bind('themeVars.borderRadius');
    width: 100%;
    height: 100%;
    border-color: transparent;
    flex-shrink: 0;
    display: none;
    align-items: center;
    padding: 1rem;
    position: absolute;
    left: 0;

    &:first-child {
        justify-content: flex-start;
        background-color: v-bind('leftColor');
    }

    &:last-child {
        justify-content: flex-end;
        background-color: v-bind('rightColor');
    }
}

.icon-grow {
    --base-scale: calc(1 + v-bind(absX) / 200);
    max-height: 100%;
    transform: scale(var(--base-scale));
}

/* Show the button when swiping */
.swiping {
    display: flex;
}

.swiped {
    display: flex;
    animation: growAndSpin .5s ease infinite;
}

@keyframes growAndSpin {
    0% {
        transform: scale(var(--base-scale));
    }

    50% {
        transform: scale(calc(var(--base-scale) * 0.65));
    }

    100% {
        transform: scale(var(--base-scale));
    }
}
</style>