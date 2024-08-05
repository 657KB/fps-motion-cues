import { createApp } from 'vue'
import { window } from '@tauri-apps/api'
import App from './App.vue'
import './main.css'

window.appWindow.setIgnoreCursorEvents(true)

createApp(App).mount('#app')
