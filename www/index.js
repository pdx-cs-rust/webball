import init from "./pkg/webball.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const webBall = await init(
    "./pkg/webball_bg.wasm"
  );
  // Run the exported function
  webBall.start();
};
runWasm();
