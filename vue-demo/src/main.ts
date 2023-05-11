import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

// Vuetify
import 'vuetify/dist/vuetify.min.css'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

import './assets/main.css'

import { PilotServicePlugin } from './pilotService/pilotServicePlugin';
import { serviceConfigurations } from './serviceConfig';

const pilotServicePlugin = new PilotServicePlugin(serviceConfigurations);

const vuetify = createVuetify({
    components,
    directives,
});

const app = createApp(App)

app.use(vuetify);
app.use(createPinia());
app.use(router);
app.use(pilotServicePlugin);

app.mount('#app');
