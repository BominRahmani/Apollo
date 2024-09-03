const { invoke } = window.__TAURI__.tauri;
import { appWindow } from '@tauri-apps/api/window';

let greetInputEl;
let greetMsgEl;

async function greet() {
  await appWindow.setSize(new LogicalSize(800, 600));
  await appWindow.center();
  searchContainerEl.classList.remove('active');
  searchContainerEl.classList.add('inactive');
  contentEl.classList.remove('hidden');
  contentEl.classList.add('visible');
  searchInputEl.value = '';
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
