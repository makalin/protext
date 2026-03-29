use wasm_bindgen_test::*;
use js_sys::Reflect;
use wasm_bindgen::JsValue;

use protext::ProtextEngine;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn measures_text_and_tracks_font() {
    let mut engine = ProtextEngine::new();
    engine.set_font("600 16px sans-serif");

    let width = engine.measure_width("Protext");

    assert_eq!(engine.font(), "600 16px sans-serif");
    assert!(width > 0.0);
}

#[wasm_bindgen_test]
fn layouts_lines_for_browser_canvas() {
    let mut engine = ProtextEngine::new();
    engine.set_font("16px sans-serif");

    let lines = engine.layout_lines(
        "Protext keeps canvas measurement honest while Rust handles wrapping.",
        140.0,
    );

    assert!(lines.length() >= 2);
}

#[wasm_bindgen_test]
fn returns_structured_measurement_and_summary() {
    let mut engine = ProtextEngine::new();
    engine.set_font("16px sans-serif");

    let measurement = engine.measure_text("alpha beta");
    let summary = engine.layout_summary("alpha beta gamma", 80.0, 20.0);

    let word_count = get_number(&measurement, "wordCount");
    let line_count = get_number(&summary, "lineCount");
    let total_height = get_number(&summary, "totalHeight");

    assert_eq!(word_count, 2.0);
    assert!(line_count >= 2.0);
    assert!(total_height >= 40.0);
}

fn get_number(value: &JsValue, key: &str) -> f64 {
    Reflect::get(value, &JsValue::from_str(key))
        .expect("property should exist")
        .as_f64()
        .expect("property should be numeric")
}
