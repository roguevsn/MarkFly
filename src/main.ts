import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './styles/global.css'

// 导入 ByteMD 样式
import 'bytemd/dist/index.css'
import 'github-markdown-css/github-markdown-light.css'
import 'highlight.js/styles/vs.css'
import 'katex/dist/katex.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.mount('#app')