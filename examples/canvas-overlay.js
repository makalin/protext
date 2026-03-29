import { formatJson, useEngine } from "./shared.js";

const fontInput = document.querySelector("#font");
const widthInput = document.querySelector("#width");
const textInput = document.querySelector("#text");
const debugOutput = document.querySelector("#debug");
const canvas = document.querySelector("#canvas");
const context = canvas.getContext("2d");

const engine = await useEngine(fontInput.value);
let phase = 0;

function paint() {
  const font = fontInput.value.trim() || "700 24px sans-serif";
  const wrapWidth = Number(widthInput.value) || 360;
  const text = textInput.value;
  const lines = Array.from(engine.layout_text(text, wrapWidth));

  context.clearRect(0, 0, canvas.width, canvas.height);
  context.fillStyle = "#f7fbfc";
  context.fillRect(0, 0, canvas.width, canvas.height);

  context.save();
  context.translate(56, 64);
  context.font = font;
  context.textBaseline = "top";
  context.strokeStyle = "#355070";
  context.lineWidth = 2;
  context.strokeRect(0, 0, wrapWidth, 28 + lines.length * 38);

  lines.forEach((line, index) => {
    const y = index * 38;
    const pulse = 0.2 + 0.2 * Math.sin(phase + index * 0.45);
    context.fillStyle = `rgba(231, 111, 81, ${0.18 + pulse})`;
    context.fillRect(0, y, line.width, 30);
    context.fillStyle = "#132238";
    context.fillText(line.text, 0, y + 4);
  });

  context.restore();

  debugOutput.textContent = formatJson({
    wrapWidth,
    lineCount: lines.length,
    maxLineWidth: engine.max_line_width(text, wrapWidth),
    summary: engine.layout_summary(text, wrapWidth, 38),
  });
}

function tick() {
  phase += 0.03;
  paint();
  requestAnimationFrame(tick);
}

fontInput.addEventListener("input", () => {
  engine.set_font(fontInput.value);
  paint();
});

widthInput.addEventListener("input", paint);
textInput.addEventListener("input", paint);

engine.set_font(fontInput.value);
paint();
requestAnimationFrame(tick);
