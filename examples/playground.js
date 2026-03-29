import { formatJson, renderLineList, sampleTexts, useEngine } from "./shared.js";

const fontInput = document.querySelector("#font");
const widthInput = document.querySelector("#width");
const lineHeightInput = document.querySelector("#line-height");
const textInput = document.querySelector("#text");
const renderButton = document.querySelector("#render");
const presetButton = document.querySelector("#preset");
const summaryOutput = document.querySelector("#summary");
const measurementOutput = document.querySelector("#measurement");
const linesOutput = document.querySelector("#lines");
const previewCopy = document.querySelector("#preview-copy");

const presets = [sampleTexts.manifesto, sampleTexts.editorial, sampleTexts.launch];
let presetIndex = 0;

const engine = await useEngine(fontInput.value);

function render() {
  const font = fontInput.value.trim() || "16px sans-serif";
  const maxWidth = Number(widthInput.value) || 280;
  const lineHeight = Number(lineHeightInput.value) || 28;
  const text = textInput.value;

  engine.set_font(font);

  const lines = Array.from(engine.layout_text(text, maxWidth));
  previewCopy.style.font = font;
  previewCopy.style.lineHeight = `${lineHeight}px`;
  previewCopy.style.maxWidth = `${maxWidth}px`;
  previewCopy.textContent = lines.map((line) => line.text).join("\n");
  previewCopy.style.whiteSpace = "pre-wrap";

  renderLineList(linesOutput, engine, lines);
  measurementOutput.textContent = formatJson(engine.measure_text(text));
  summaryOutput.textContent = formatJson(engine.layout_summary(text, maxWidth, lineHeight));
}

renderButton.addEventListener("click", render);
fontInput.addEventListener("input", render);
widthInput.addEventListener("input", render);
lineHeightInput.addEventListener("input", render);

presetButton.addEventListener("click", () => {
  presetIndex = (presetIndex + 1) % presets.length;
  textInput.value = presets[presetIndex];
  render();
});

render();
