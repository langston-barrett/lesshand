# Try it!

Try Lesshand here! Type plain English in the upper box, then hit "Encode" to
compress it into Lesshand. Alternatively, type Lesshand in the lower box and hit
"Decode" to decompress it into English.

<script type="module" src="./wasm/demo.js"></script>

<style>
#demo-wrap { display: grid; grid-template-columns: 1fr; gap: 0.75rem; align-items: stretch; }
#demo-wrap textarea { width: 100%; min-height: 200px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; font-size: 14px; padding: 8px; }
#demo-wrap .buttons { display: flex; gap: 0.75rem; justify-content: flex-start; }
#demo-wrap button { padding: 0.5rem 0.75rem; font-weight: 600; }
#demo-wrap h3 { margin-top: 0; }
</style>

<div id="demo-wrap">
  <div>
    <h3>English</h3>
    <textarea id="input-area">You are really going to enjoy your day tomorrow, it&apos;ll be very special.</textarea>
  </div>
  <div class="buttons">
    <button id="btn-enc" disabled>Encode ↓</button>
    <button id="btn-dec" disabled>Decode ↑</button>
  </div>
  <div>
    <h3>Lesshand</h3>
    <textarea id="output-area" placeholder="Result appears here"></textarea>
  </div>
</div>
