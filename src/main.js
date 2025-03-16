const { invoke } = window.__TAURI__.core;

// wasm init
import wasm_init, { simple_init } from './wasm/artimonist_wasm.js';
async function run() {
  await wasm_init();
}
run();

// generate 
let diagram = document.querySelector('simple-diagram');
let password = document.querySelector('unicode-password');
let generate = document.querySelector('#generate');
generate.addEventListener('click', async () => {
  if (diagram.is_empty() || !password.is_valid()) {
    return;
  }
  let xpriv = await simple_init(["1", "2", "3", "4", "5", "6", "7", "8", "9"], "xxx");
  console.log(xpriv);
});

diagram.addEventListener('onchange', e => {
  console.log(e.target.values);
  console.log('empty: ' + diagram.is_empty());
  console.log('password: ' + password.is_valid());
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  // greetInputEl = document.querySelector("#greet-input");
  // greetMsgEl = document.querySelector("#greet-msg");
  // document.querySelector("#greet-form").addEventListener("submit", (e) => {
  //   e.preventDefault();
  //   greet();
  // });
});
