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
                <n-button class="under-swipe-part" :class="[{ 'swiping': isSwipingLeft }]" primary type="warning"
                    :bordered="false">
                    <TimerIcon class="icon-small icon-grow"
                        :class="[{ 'swiped': isSwipedLeft, 'swiping': isSwipingLeft }]" />
                </n-button>
                <n-button class="under-swipe-part" :class="[{ 'swiping': isSwipingRight }]" primary type="info"
                    :bordered="false">
                    <WaterIcon class="icon-small icon-grow" :class="[{ 'swiped': isSwipedRight }]" />
                </n-button>
            </div>
        </n-flex>
    </n-collapse-transition>
</template>

<script setup lang="ts">
import { useMessage, useThemeVars } from 'naive-ui';
import { Timer16Regular as TimerIcon, Drop20Regular as WaterIcon } from '@vicons/fluent'
import { ref } from 'vue';

const themeVars = useThemeVars();
const message = useMessage();

// min and max heigh of icone under the main structure
const animationTime = 0.5;
const threshold = 150;

// vue ref
const isSwipedRight = ref(false);
const isSwipedLeft = ref(false);
const isSwipingRight = ref(false);
const isSwipingLeft = ref(false);
const absX = ref(0);
const startX = ref(0);
const currentX = ref(0);
const isDragging = ref(false);
const emits = defineEmits(['swipedLeft', 'swipedRight'])
const show = ref(true);

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
    if (Math.abs(deltaX) <= threshold) {
        currentX.value = deltaX;
        absX.value = Math.abs(deltaX);
    }

    if (deltaX < 0) {
        isSwipingRight.value = true;
    } else {
        isSwipingLeft.value = true;
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
        isSwipingLeft.value = false;
        isSwipingRight.value = false;
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
    border-radius: v-bind('themeVars.borderRadius');
    z-index: 0;
    flex-flow: nowrap;
    margin: 0;
    padding: 0;
}

.under-swipe-part {
    box-sizing: border-box;
    width: 100%;
    height: 100%;
    border-color: transparent;
    flex-shrink: 0;
    display: none;
    align-items: center;
    padding: 1rem;
    position: absolute;
    left: 0;
}

/* First button (left swipe) */
.under-swipe-part:first-child {
    justify-content: flex-start;
}

/* Second button (right swipe) */
.under-swipe-part:last-child {
    justify-content: flex-end;
}

/* Show the button when swiping */
.swiping {
    display: flex;
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