import { sampleTexts, useEngine } from "./shared.js";

const titleInput = document.querySelector("#title");
const kickerInput = document.querySelector("#kicker");
const fontInput = document.querySelector("#font");
const widthInput = document.querySelector("#width");
const renderButton = document.querySelector("#render");
const posterKicker = document.querySelector("#poster-kicker");
const posterTitle = document.querySelector("#poster-title");
const posterLines = document.querySelector("#poster-lines");

const engine = await useEngine(fontInput.value);

function render() {
  const title = titleInput.value.trim() || "Signals for the next release";
  const kicker = kickerInput.value.trim() || "Experimental launch system";
  const font = fontInput.value.trim() || "700 28px Georgia, serif";
  const width = Number(widthInput.value) || 320;

  engine.set_font(font);

  const lines = Array.from(engine.layout_lines(title, width));

  posterKicker.textContent = kicker;
  posterTitle.style.font = font;
  posterTitle.style.maxWidth = `${width}px`;
  posterTitle.textContent = title;
  posterLines.innerHTML = "";

  for (const line of lines) {
    const capsule = document.createElement("div");
    capsule.className = "poster-line";
    capsule.textContent = line;
    posterLines.appendChild(capsule);
  }
}

renderButton.addEventListener("click", render);
titleInput.addEventListener("input", render);
kickerInput.addEventListener("input", render);
fontInput.addEventListener("input", render);
widthInput.addEventListener("input", render);

titleInput.value ||= sampleTexts.launch;
render();
