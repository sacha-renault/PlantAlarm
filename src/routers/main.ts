import { createMemoryHistory, createRouter } from 'vue-router'
import CalendarView from '../views/CalendarView.vue'

const routes = [
    { path: '/', component: CalendarView },
    { path: '/calendar', component: CalendarView },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})