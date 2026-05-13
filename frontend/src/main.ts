import { createApp } from "vue";
import App from "./App.vue";
import { pinia } from "./store";
import { useThemeStore } from "./store/theme";
import "./styles/theme.css";

const app = createApp(App);
app.use(pinia);

const theme = useThemeStore();
theme.init();

app.mount("#app");
