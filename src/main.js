// CSS
import './assets/main.css'
import 'primevue/resources/themes/soho-dark/theme.css'
import 'primevue/resources/primevue.min.css'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'
// Libs
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import { RegisterComponents } from './registration.js'

import App from './App.vue'

const app = createApp(App)
// Registration
app.use(createPinia())
app.use(PrimeVue, { ripple: true })
RegisterComponents(app)

app.mount('#app')
