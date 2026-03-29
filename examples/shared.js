import init, { ProtextEngine } from "../pkg/protext.js";

export const sampleTexts = {
  manifesto:
    "Protext keeps browser typography visually honest by measuring with Canvas and shaping layout decisions in Rust.",
  editorial:
    "A release-ready text engine needs more than one playground. It needs demos for reading, animation, overlays, and responsive stress tests.",
  launch:
    "Design systems ship faster when text measurement, wrapping, and preview instrumentation are consistent across every surface.",
  poster:
    "Signals / Stories / Systems / Motion / Resize / Measure / Release",
};

const enginePromise = init().then(() => new ProtextEngine());

export async function useEngine(font) {
  const engine = await enginePromise;

  if (font) {
    engine.set_font(font);
  }

  return engine;
}

export function formatJson(value) {
  return JSON.stringify(value, null, 2);
}

export function renderLineList(container, engine, lines) {
  container.innerHTML = "";

  for (const line of lines) {
    const item = document.createElement("li");
    item.className = "line-chip";

    const text = document.createElement("span");
    text.textContent = line.text ?? line;

    const width = document.createElement("strong");
    const measured = typeof line === "string" ? engine.measure_width(line) : line.width;
    width.textContent = `${measured.toFixed(2)}px`;

    item.append(text, width);
    container.appendChild(item);
  }
}

export function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

export function lerp(a, b, t) {
  return a + (b - a) * t;
}
