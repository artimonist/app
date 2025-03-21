const { invoke } = window.__TAURI__.core;

let diagram = document.querySelector('simple-diagram');
let password = document.querySelector('unicode-password');
let generate = document.querySelector('#generate');
let overlay = document.getElementById('overlay');

// worker
let worker = new Worker('./worker/worker.js');
worker.onmessage = function (event) {
  overlay.style.display = 'none';
  console.log('Received from worker:', event.data);
};

// generate 
generate.addEventListener('click', async () => {
  if (diagram.is_empty() || !password.is_valid()) {
    return;
  }
  overlay.style.display = 'block';
  worker.postMessage({
    method: 'simple_init',
    args: [diagram.values, password.value]
  });
});

diagram.addEventListener('onchange', e => {
});
