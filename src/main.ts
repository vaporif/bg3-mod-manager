import "./styles.css";
import App from "./App.svelte";

const appElement = document.getElementById("app");
if (!appElement) throw new Error('app element not found');

const app = new App({
  target: appElement
});

export default app;
