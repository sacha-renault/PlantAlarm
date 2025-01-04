<script setup lang="ts">
import { darkTheme, lightTheme } from 'naive-ui'
import { ref } from 'vue'
import { useThemeStore } from './stores/theme'
import MainLayout from './views/MainLayout.vue';

// Theme handling
const themeStore = useThemeStore();
const isDarkTheme = ref(themeStore.isDark);
themeStore.$subscribe((_, state) => {
  console.log('Theme changed:', state.isDark)
  isDarkTheme.value = state.isDark;
})

</script>

<template>
  <n-config-provider :theme="isDarkTheme ? darkTheme : lightTheme">
    <n-modal-provider>
      <n-message-provider>
        <main-layout />
        <n-global-style />
      </n-message-provider>
    </n-modal-provider>
  </n-config-provider>
</template>