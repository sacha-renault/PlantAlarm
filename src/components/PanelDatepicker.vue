<template>
    <div class="date-picker" :class="{ 'error-date-picker' : isError, 'warning-date-picker': isWarning }" :data-msg="msg">
        <n-date-picker
            panel type="date"
            v-model:value="model"/>
    </div>
</template>

<script setup lang="ts">
import { useThemeVars } from 'naive-ui';
import { computed } from 'vue';

const themeVars = useThemeVars();
const model = defineModel<Number>({
  required: true
});
const { isError, errMsg, isWarning, warnMsg } = defineProps({
    isError: {
        type: Boolean,
        default: false
    },
    errMsg: {
        type: String,
        default: 'Date is invalid: shouldn\'t be future'
    },
    isWarning: {
        type: Boolean,
        default: false
    },
    warnMsg: {
        type: String,
        default: 'Careful about future date.'
    },
});

const msg = computed(() => {
    if (isError) {
        return errMsg;
    } else if (isWarning) {
        return warnMsg;
    } else {
        return '';
    }
})

</script>

<style scoped lang="scss">

.date-picker {
    position: relative;
    box-sizing: border-box;
    border-radius: v-bind('themeVars.borderRadius');
    border: solid 1px;
    border-color: transparent;
    transition: 0.05s;

    &::after {
        position: absolute;
        background-color: v-bind('themeVars.popoverColor');
        bottom: 0;
        left: 25%;
        transform: translate(-25%, 50%);
        content: attr(data-msg);
    }

    &.error-date-picker {
        border-color: v-bind('themeVars.errorColor');

        &::after{
            color: v-bind('themeVars.errorColor');
        }
    }

    &.warning-date-picker {
        border-color: v-bind('themeVars.warningColor');

        &::after{
            color: v-bind('themeVars.warningColor');
        }
    }
}

</style>