import { createMemoryHistory, createRouter } from 'vue-router'
import CalendarView from '../views/CalendarView.vue'
import ListView from '../views/ListView.vue'
import PlantsManagementView from '../views/PlantsManagementView.vue'

const routes = [
    { path: '/', component: ListView },
    { path: '/calendar', component: CalendarView },
    { path: '/list', component: ListView },
    { path: '/plants', component: PlantsManagementView },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})