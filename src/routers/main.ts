import { createMemoryHistory, createRouter } from 'vue-router'
import CalendarView from '../views/CalendarView.vue'
import ListView from '../views/ListView.vue'

const routes = [
    { path: '/', component: CalendarView },
    { path: '/calendar', component: CalendarView },
    { path: '/list', component: ListView },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})