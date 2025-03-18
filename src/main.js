const { invoke } = window.__TAURI__.core;

// worker
var worker = new Worker('./worker/worker.js');
worker.onmessage = function (event) {
  // document.body.style.cursor = 'default';
  $('*').css('cursor', 'default');
  console.log('Received from worker:', event.data);
};

// generate 
let diagram = document.querySelector('simple-diagram');
let password = document.querySelector('unicode-password');
let generate = document.querySelector('#generate');
generate.addEventListener('click', async () => {
  if (diagram.is_empty() || !password.is_valid()) {
    return;
  }
  $('*').css('cursor', 'wait');
  document.body.style.cursor = 'wait';
  worker.postMessage({
    method: 'simple_init',
    args: [diagram.values, password.value]
  });
});

diagram.addEventListener('onchange', e => {
});
