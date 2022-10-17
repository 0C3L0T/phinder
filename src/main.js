const { invoke } = window.__TAURI__.tauri;

let userInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  userInputEl = document.querySelector("#user-input");
});

function clickPress(event) {
  if (event.key === "Enter") {
    console.log("Enter pressed");
    handle_input()
  }
  if (event.key === "Escape") {
    kill()
  }
}

async function kill() {
  await invoke("kill");
}

async function handle_input() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("handle_input", { input: userInputEl.value });
  // clearn input
  userInputEl.value = "";
}

window.clickPress = clickPress;
