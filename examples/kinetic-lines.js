import { clamp, formatJson, sampleTexts, useEngine } from "./shared.js";

const fontInput = document.querySelector("#font");
const textInput = document.querySelector("#text");
const toggleButton = document.querySelector("#toggle");
const resetButton = document.querySelector("#reset");
const stage = document.querySelector("#stage");
const stats = document.querySelector("#stats");

const engine = await useEngine(fontInput.value);
let running = true;
let phase = 0;

function renderFrame() {
  const font = fontInput.value.trim() || "700 32px sans-serif";
  const text = textInput.value || sampleTexts.editorial;
  const width = 180 + (Math.sin(phase) + 1) * 150;

  engine.set_font(font);

  const lines = Array.from(engine.layout_text(text, width));
  stage.innerHTML = "";

  lines.forEach((line, index) => {
    const div = document.createElement("div");
    div.className = "kinetic-line visible";
    div.style.top = `${26 + index * 58}px`;
    div.style.font = font;
    div.textContent = line.text;
    stage.appendChild(div);
  });

  stats.textContent = formatJson({
    width: Number(width.toFixed(2)),
    lineCount: lines.length,
    maxLineWidth: engine.max_line_width(text, width),
    estimateHeight: engine.estimate_height(text, width, 42),
  });
}

function animate() {
  if (running) {
    phase += 0.035;
    if (phase > Math.PI * 2) {
      phase = 0;
    }
    renderFrame();
  }
  requestAnimationFrame(animate);
}

toggleButton.addEventListener("click", () => {
  running = !running;
  toggleButton.textContent = running ? "Pause" : "Resume";
});

resetButton.addEventListener("click", () => {
  phase = 0;
  renderFrame();
});

fontInput.addEventListener("input", renderFrame);
textInput.addEventListener("input", () => {
  if (textInput.value.trim().length === 0) {
    textInput.value = sampleTexts.editorial;
  }
  renderFrame();
});

renderFrame();
requestAnimationFrame(animate);
