importScripts('./artimonist_wasm.js');
console.log('Initializing worker')

const { simple_init, complex_init, generate, compress, encrypt } = wasm_bindgen;
const proxy = {
  'simple_init': simple_init,
  'complex_init': complex_init,
  'generate': generate,
  'compress': compress,
  'encrypt': encrypt,
}

async function init_wasm_in_worker() {
  await wasm_bindgen('./artimonist_wasm_bg.wasm');

  // dispatch method calls from main thread
  self.onmessage = async evt => {
    let msg = evt.data;
    let result = proxy[msg.method](...msg.args);
    self.postMessage(result);
  };
};
init_wasm_in_worker();
