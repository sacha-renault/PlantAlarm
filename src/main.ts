import { createApp } from "vue";
import App from "./App.vue";
import naive from 'naive-ui'
import { createPinia } from "pinia";
import './assets/main.css'
import { router } from './routers/main.ts'

const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.use(naive);
app.use(router);
app.mount("#app");
