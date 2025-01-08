<template>
    <n-flex class="wf main-swipable-container" ref="mainContainer">
        <!-- Main display -->
        <div @touchstart="touchHandlers.onTouchStart"
            @touchmove="touchHandlers.onTouchMove" @touchend="touchHandlers.onTouchEnd"
            @mousedown="mouseHandlers.onMouseDown" @mousemove="mouseHandlers.onMouseMove"
            @mouseup="mouseHandlers.onMouseUp" :style="{ transform: 'translateX(' + currentX.toString() + 'px)' }"
            class="wf swipable-container" :class="{'swipable-container-swiping' : isDragging }">
            <slot name="default"/>
        </div>

        <!-- Under swipe -->
        <div class="under-swipe-container">

            <!-- Left part -->
            <div class="under-swipe-part" :class="[{ 'under-swipe-visible': isSwipingLeft }]">
                <div class="icon-small icon-grow" :class="[{ 'swiped': isSwipedLeft }]">
                    <slot name="icon-left"/>
                </div>
            </div>

            <!-- Right part -->
            <div class="under-swipe-part " :class="[{ 'under-swipe-visible': isSwipingRight }]">
                <div class="icon-small icon-grow" :class="[{ 'swiped': isSwipedRight }]">
                    <slot name="icon-right"/>
                </div>
            </div>
        </div>
    </n-flex>
</template>

<script lang="ts">
import { useThemeVars } from 'naive-ui';

export default {
    props: {
        leftColor: {
            type: String,
        },
        rightColor: {
            type: String,
        },
        animationDuration: {
            type: Number,
            default: 0.5
        },
        maxSwipe: {
            type: String,
            default: '50%'
        },
        swipeThreshold: {
            type: String,
            default: '45%'
        },
        swipeAnimation: {
            type: String,
            default: 'growAndSpin'
        },
        replaceTransitionDuration: {
            type: Number,
            default: 0.3
        }
    },
    methods: {
        // Swip methods
        handleDragStart(x: number) {
            this.startX = x;
            this.isDragging = true;
        },

        handleDragMove(x: number) {
            if (!this.isDragging) return;

            const deltaX = x - this.startX;
            const absDeltaX = Math.abs(deltaX);

            // Handle not swiping too far
            if (this.maxSwipePx > absDeltaX) {
                this.absX = absDeltaX;
                this.currentX = deltaX;
            }

            // Handle direction swipe
            if (deltaX < 0) {
                this.isSwipingRight = true;
                this.isSwipingLeft = false;
            } else if (deltaX > 0) {
                this.isSwipingLeft = true;
                this.isSwipingRight = false;
            }
        },

        handleDragEnd () {
            this.isDragging = false;

            if (Math.abs(this.currentX) < this.thresholdPx) {
                this.currentX = 0;
                return;
            }

            if (this.currentX < 0) {
                this.isSwipedRight = true;
                this.$emit('swipedRight');
            } else {
                this.isSwipedLeft = true;
                this.$emit('swipedLeft');
            }

            setTimeout(() => {
                this.currentX = 0;
                this.isSwipedLeft = false;
                this.isSwipedRight = false;
                this.$emit('swipeAnimationOver');

                setTimeout(() => {
                    this.isSwipingRight = false;
                    this.isSwipingLeft = false;
                }, this.replaceTransitionDuration * 1000);
            }, this.animationDuration * 1000);
        },

        parseStringAsPx(value: string): number {
            const element = this.$el;
            if (value.endsWith('px')) {
                return parseInt(value.replace('px', ''));
            } else if (value.endsWith('%')) {
                const elementWidth = element ? element.clientWidth : 0;
                return (parseInt(value.replace('%', ''), 10) / 100) * elementWidth;
            } else {
                return 0;
            }
        }
    },
    data() {
        return  {
            mouseHandlers: {
                onMouseDown: (e: MouseEvent) => this.handleDragStart(e.clientX),
                onMouseMove: (e: MouseEvent) => this.handleDragMove(e.clientX),
                onMouseUp: () => this.handleDragEnd(),
            },
            touchHandlers: {
                onTouchStart: (e: TouchEvent) => this.handleDragStart(e.touches[0].clientX),
                onTouchMove: (e: TouchEvent) => this.handleDragMove(e.touches[0].clientX),
                onTouchEnd: () => this.handleDragEnd()
            },
            themeVars: useThemeVars(),
            isSwipedRight: false,
            isSwipedLeft: false,
            currentX: 0,
            absX: 0,
            startX: 0,
            isDragging: false,
            isSwipingRight: false,
            isSwipingLeft: false
        }
    },
    computed: {
        leftUnderColor() { return this.leftColor ?? this.themeVars.warningColor; },
        rightUnderColor(){ return this.rightColor ?? this.themeVars.infoColor; },
        maxSwipePx() { return this.parseStringAsPx(this.maxSwipe) },
        thresholdPx() { return this.parseStringAsPx(this.swipeThreshold) }
    }
}

</script>

<style scoped>
.main-swipable-container {
    position: relative;
    width: 100%;
    height: 100%;
    cursor: grab;
}

.swipable-container {
    --transition-duration: v-bind('replaceTransitionDuration');
    padding: 0.5rem 1rem;
    border: 1px solid;
    border-radius: v-bind('themeVars.borderRadius');
    border-color: v-bind('themeVars.borderColor');
    background-color: v-bind('themeVars.bodyColor');
    z-index: 100;
    transition: calc(var(--transition-duration) * 1s);
}

.swipable-container-swiping {
    transition: unset;
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
        background-color: v-bind('leftUnderColor');
    }

    &:last-child {
        justify-content: flex-end;
        background-color: v-bind('rightUnderColor');
    }
}

.icon-grow {
    --base-scale: calc(1 + v-bind(absX) / 200);
    max-height: 100%;
    transform: scale(var(--base-scale));
}

/* Show the button when swiping */
.under-swipe-visible {
    display: flex;
}

.swiped {
    --animation-duration: v-bind('animationDuration');
    display: flex;
    animation-name: growAndSpin;
    animation-duration: calc(var(--animation-duration) * 1s);
    animation-iteration-count: infinite;
    animation-timing-function: ease;
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