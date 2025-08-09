import init, { encode, decode } from './pkg/lesshand_wasm.js';

function $(id) { return document.getElementById(id); }

async function main() {
  await init();
  const encBtn = $('btn-enc');
  const decBtn = $('btn-dec');
  encBtn.disabled = false;
  decBtn.disabled = false;

  encBtn.addEventListener('click', () => {
    const src = /** @type {HTMLTextAreaElement} */ ($('input-area')).value;
    /** @type {HTMLTextAreaElement} */ ($('output-area')).value = encode(src);
  });
  decBtn.addEventListener('click', () => {
    const out = /** @type {HTMLTextAreaElement} */ ($('output-area')).value;
    /** @type {HTMLTextAreaElement} */ ($('input-area')).value = decode(out);
  });
}

main();
