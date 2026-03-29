import { formatJson, useEngine } from "./shared.js";

const fontInput = document.querySelector("#font");
const textInput = document.querySelector("#text");
const target = document.querySelector("#target");
const summaryOutput = document.querySelector("#summary");
const sparkline = document.querySelector("#sparkline");

const engine = await useEngine(fontInput.value);
const samples = [];

function recordSample(width, summary) {
  samples.push({ width, lineCount: summary.lineCount });
  if (samples.length > 12) {
    samples.shift();
  }

  sparkline.innerHTML = "";

  for (const sample of samples) {
    const bar = document.createElement("div");
    bar.className = "bar";
    bar.style.height = `${Math.max(22, sample.lineCount * 20)}px`;
    const label = document.createElement("span");
    label.textContent = `${sample.width}px`;
    bar.appendChild(label);
    sparkline.appendChild(bar);
  }
}

function render(width) {
  const font = fontInput.value.trim() || "700 18px sans-serif";
  const text = textInput.value;

  engine.set_font(font);
  target.style.font = font;
  target.textContent = Array.from(engine.layout_lines(text, width)).join("\n");
  target.style.whiteSpace = "pre-wrap";

  const summary = engine.layout_summary(text, width, 28);
  summaryOutput.textContent = formatJson({ width, ...summary });
  recordSample(width, summary);
}

const observer = new ResizeObserver((entries) => {
  const entry = entries[0];
  const width = Math.floor(entry.contentRect.width - 44);
  render(Math.max(width, 120));
});

observer.observe(target);

fontInput.addEventListener("input", () => render(Math.floor(target.clientWidth - 44)));
textInput.addEventListener("input", () => render(Math.floor(target.clientWidth - 44)));
