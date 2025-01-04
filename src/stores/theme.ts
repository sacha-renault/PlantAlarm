// stores/theme.ts
import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        isDark: true
    }),
    actions: {
        toggleTheme() {
            this.isDark = !this.isDark
        },
        setTheme(isDark: boolean) {
            this.isDark = isDark
        }
    },
    getters: {
        currentTheme: (state) => state.isDark ? 'dark' : 'light'
    }
})