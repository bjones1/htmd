// Loads the converter and the user interface as two separate wasm
// instances. A crash in the converter (a panic, or a stack overflow on deeply
// nested input) unwinds only its own instance, so the interface survives to
// report it. The interface calls the converter through this global
// (app/src/engine.rs).
import initEngine, * as engine from "./pkg/engine.js";
import initApp from "./pkg/app.js";

try {
  await initEngine();
  globalThis.htmdPlaygroundEngine = engine;
  await initApp();
} catch (error) {
  const message = document.createElement("p");
  message.className = "load-error";
  message.textContent = `The playground failed to load: ${error}`;
  document.body.append(message);
  throw error;
}
