use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Debug, PartialEq)]
struct LayoutLine {
    text: String,
    width: f64,
    paragraph_index: usize,
    is_hard_break: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct LayoutSummary {
    line_count: usize,
    max_line_width: f64,
    total_height: f64,
    paragraph_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenKind {
    Word,
    Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    text: String,
    kind: TokenKind,
}

#[wasm_bindgen]
pub struct ProtextEngine {
    context: CanvasRenderingContext2d,
    font: String,
}

#[wasm_bindgen]
impl ProtextEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ProtextEngine {
        let context = create_canvas_context()
            .expect("ProtextEngine requires a browser environment with Canvas 2D support");
        let font = String::from("16px sans-serif");

        context.set_font(&font);

        ProtextEngine { context, font }
    }

    pub fn set_font(&mut self, font: &str) {
        self.font.clear();
        self.font.push_str(font);
        self.context.set_font(&self.font);
    }

    pub fn font(&self) -> String {
        self.font.clone()
    }

    pub fn measure_width(&self, text: &str) -> f64 {
        self.context
            .measure_text(text)
            .map(|metrics| metrics.width())
            .unwrap_or(0.0)
    }

    pub fn measure_text(&self, text: &str) -> JsValue {
        text_measurement_to_js(text, self.measure_width(text))
    }

    pub fn layout_lines(&self, text: &str, max_width: f64) -> Array {
        let result = Array::new();

        for line in layout_text_with(text, max_width, |value| self.measure_width(value)) {
            result.push(&JsValue::from(line.text));
        }

        result
    }

    pub fn layout_text(&self, text: &str, max_width: f64) -> Array {
        let result = Array::new();

        for line in self.layout_text_internal(text, max_width) {
            result.push(&layout_line_to_js(&line));
        }

        result
    }

    pub fn line_count(&self, text: &str, max_width: f64) -> usize {
        self.layout_text_internal(text, max_width).len()
    }

    pub fn max_line_width(&self, text: &str, max_width: f64) -> f64 {
        self.layout_text_internal(text, max_width)
            .iter()
            .map(|line| line.width)
            .fold(0.0, f64::max)
    }

    pub fn estimate_height(&self, text: &str, max_width: f64, line_height: f64) -> f64 {
        let summary = summarize_layout(
            &self.layout_text_internal(text, max_width),
            normalized_line_height(line_height),
        );

        summary.total_height
    }

    pub fn layout_summary(&self, text: &str, max_width: f64, line_height: f64) -> JsValue {
        let summary = summarize_layout(
            &self.layout_text_internal(text, max_width),
            normalized_line_height(line_height),
        );

        layout_summary_to_js(&summary)
    }
}

impl ProtextEngine {
    fn layout_text_internal(&self, text: &str, max_width: f64) -> Vec<LayoutLine> {
        layout_text_with(text, max_width, |value| self.measure_width(value))
    }
}

fn create_canvas_context() -> Result<CanvasRenderingContext2d, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window is unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document is unavailable"))?;
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("2D rendering context is unavailable"))?;

    Ok(context.dyn_into::<CanvasRenderingContext2d>()?)
}

fn layout_line_to_js(line: &LayoutLine) -> JsValue {
    let object = Object::new();

    set_js_prop(&object, "text", JsValue::from(line.text.clone()));
    set_js_prop(&object, "width", JsValue::from_f64(line.width));
    set_js_prop(
        &object,
        "paragraphIndex",
        JsValue::from_f64(line.paragraph_index as f64),
    );
    set_js_prop(&object, "hardBreak", JsValue::from_bool(line.is_hard_break));

    JsValue::from(object)
}

fn text_measurement_to_js(text: &str, width: f64) -> JsValue {
    let object = Object::new();

    set_js_prop(&object, "text", JsValue::from(text.to_owned()));
    set_js_prop(&object, "width", JsValue::from_f64(width));
    set_js_prop(
        &object,
        "characterCount",
        JsValue::from_f64(text.chars().count() as f64),
    );
    set_js_prop(
        &object,
        "wordCount",
        JsValue::from_f64(count_words(text) as f64),
    );
    set_js_prop(
        &object,
        "paragraphCount",
        JsValue::from_f64(paragraph_count(text) as f64),
    );
    set_js_prop(
        &object,
        "empty",
        JsValue::from_bool(text.trim().is_empty()),
    );

    JsValue::from(object)
}

fn layout_summary_to_js(summary: &LayoutSummary) -> JsValue {
    let object = Object::new();

    set_js_prop(
        &object,
        "lineCount",
        JsValue::from_f64(summary.line_count as f64),
    );
    set_js_prop(
        &object,
        "maxLineWidth",
        JsValue::from_f64(summary.max_line_width),
    );
    set_js_prop(
        &object,
        "totalHeight",
        JsValue::from_f64(summary.total_height),
    );
    set_js_prop(
        &object,
        "paragraphCount",
        JsValue::from_f64(summary.paragraph_count as f64),
    );

    JsValue::from(object)
}

fn set_js_prop(target: &Object, key: &str, value: JsValue) {
    Reflect::set(target, &JsValue::from_str(key), &value)
        .expect("setting JS property should not fail");
}

fn summarize_layout(lines: &[LayoutLine], line_height: f64) -> LayoutSummary {
    LayoutSummary {
        line_count: lines.len(),
        max_line_width: lines.iter().map(|line| line.width).fold(0.0, f64::max),
        total_height: lines.len() as f64 * line_height,
        paragraph_count: lines
            .iter()
            .map(|line| line.paragraph_index)
            .max()
            .map_or(0, |index| index + 1),
    }
}

fn normalized_line_height(line_height: f64) -> f64 {
    if line_height.is_finite() && line_height > 0.0 {
        line_height
    } else {
        16.0
    }
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn paragraph_count(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.split('\n').count()
    }
}

fn layout_text_with<F>(text: &str, max_width: f64, measure: F) -> Vec<LayoutLine>
where
    F: Fn(&str) -> f64,
{
    if text.is_empty() {
        return Vec::new();
    }

    let unconstrained = !max_width.is_finite() || max_width <= 0.0;
    let paragraphs: Vec<&str> = text.split('\n').collect();
    let mut lines = Vec::new();

    for (paragraph_index, paragraph) in paragraphs.iter().enumerate() {
        let is_last_paragraph = paragraph_index + 1 == paragraphs.len();

        if paragraph.is_empty() {
            lines.push(LayoutLine {
                text: String::new(),
                width: 0.0,
                paragraph_index,
                is_hard_break: !is_last_paragraph,
            });
            continue;
        }

        if unconstrained {
            lines.push(LayoutLine {
                text: (*paragraph).to_owned(),
                width: measure(paragraph),
                paragraph_index,
                is_hard_break: !is_last_paragraph,
            });
            continue;
        }

        let mut current = String::new();
        let mut current_width = 0.0;

        for token in tokenize_paragraph(paragraph) {
            if current.is_empty() && token.kind == TokenKind::Whitespace {
                continue;
            }

            if current.is_empty() && token.kind == TokenKind::Word {
                let token_width = measure(&token.text);

                if token_width > max_width {
                    let segments = break_long_token(&token.text, max_width, &measure);

                    for segment in segments.iter().take(segments.len().saturating_sub(1)) {
                        push_line(
                            &mut lines,
                            segment,
                            measure(segment),
                            paragraph_index,
                            false,
                        );
                    }

                    if let Some(last) = segments.last() {
                        current = last.clone();
                        current_width = measure(&current);
                    }

                    continue;
                }
            }

            let candidate = format!("{current}{}", token.text);
            let candidate_width = measure(&candidate);

            if current.is_empty() || candidate_width <= max_width {
                current = candidate;
                current_width = candidate_width;
                continue;
            }

            push_line(&mut lines, &current, current_width, paragraph_index, false);
            current.clear();
            current_width = 0.0;

            match token.kind {
                TokenKind::Whitespace => {}
                TokenKind::Word => {
                    if measure(&token.text) <= max_width {
                        current = token.text;
                        current_width = measure(&current);
                    } else {
                        let segments = break_long_token(&token.text, max_width, &measure);

                        for segment in segments.iter().take(segments.len().saturating_sub(1)) {
                            push_line(
                                &mut lines,
                                segment,
                                measure(segment),
                                paragraph_index,
                                false,
                            );
                        }

                        if let Some(last) = segments.last() {
                            current = last.clone();
                            current_width = measure(&current);
                        }
                    }
                }
            }
        }

        if !current.is_empty() {
            push_line(
                &mut lines,
                &current,
                current_width,
                paragraph_index,
                !is_last_paragraph,
            );
        } else if !is_last_paragraph && lines.last().is_none_or(|line| line.paragraph_index != paragraph_index) {
            lines.push(LayoutLine {
                text: String::new(),
                width: 0.0,
                paragraph_index,
                is_hard_break: true,
            });
        }
    }

    lines
}

fn push_line(
    lines: &mut Vec<LayoutLine>,
    text: &str,
    width: f64,
    paragraph_index: usize,
    is_hard_break: bool,
) {
    lines.push(LayoutLine {
        text: text.trim_end().to_owned(),
        width,
        paragraph_index,
        is_hard_break,
    });
}

fn tokenize_paragraph(paragraph: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut current_kind = None;

    for ch in paragraph.chars() {
        let next_kind = if ch.is_whitespace() {
            TokenKind::Whitespace
        } else {
            TokenKind::Word
        };

        if current_kind == Some(next_kind) {
            current.push(ch);
            continue;
        }

        if !current.is_empty() {
            tokens.push(Token {
                text: current,
                kind: current_kind.expect("token kind should exist"),
            });
        }

        current = ch.to_string();
        current_kind = Some(next_kind);
    }

    if !current.is_empty() {
        tokens.push(Token {
            text: current,
            kind: current_kind.expect("token kind should exist"),
        });
    }

    tokens
}

fn break_long_token<F>(token: &str, max_width: f64, measure: &F) -> Vec<String>
where
    F: Fn(&str) -> f64,
{
    let mut segments = Vec::new();
    let mut current = String::new();

    for ch in token.chars() {
        let candidate = format!("{current}{ch}");

        if current.is_empty() || measure(&candidate) <= max_width {
            current = candidate;
            continue;
        }

        segments.push(current);
        current = ch.to_string();
    }

    if !current.is_empty() {
        segments.push(current);
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::{
        count_words, layout_text_with, normalized_line_height, paragraph_count, summarize_layout,
        tokenize_paragraph, Token, TokenKind,
    };

    fn width(value: &str) -> f64 {
        value.chars().count() as f64
    }

    #[test]
    fn tokenizes_words_and_spaces() {
        let tokens = tokenize_paragraph("alpha  beta");

        assert_eq!(
            tokens,
            vec![
                Token {
                    text: "alpha".into(),
                    kind: TokenKind::Word,
                },
                Token {
                    text: "  ".into(),
                    kind: TokenKind::Whitespace,
                },
                Token {
                    text: "beta".into(),
                    kind: TokenKind::Word,
                },
            ]
        );
    }

    #[test]
    fn wraps_text_without_exceeding_width() {
        let lines = layout_text_with("alpha beta gamma", 10.0, width);

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text, "alpha beta");
        assert_eq!(lines[1].text, "gamma");
        assert!(lines.iter().all(|line| line.width <= 10.0));
    }

    #[test]
    fn splits_long_words_when_needed() {
        let lines = layout_text_with("supercalifragilistic", 5.0, width);
        let texts: Vec<&str> = lines.iter().map(|line| line.text.as_str()).collect();

        assert_eq!(texts, vec!["super", "calif", "ragil", "istic"]);
    }

    #[test]
    fn preserves_explicit_paragraph_breaks() {
        let lines = layout_text_with("alpha\n\nbeta", 50.0, width);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].text, "alpha");
        assert!(lines[0].is_hard_break);
        assert_eq!(lines[1].text, "");
        assert!(lines[1].is_hard_break);
        assert_eq!(lines[2].text, "beta");
        assert!(!lines[2].is_hard_break);
    }

    #[test]
    fn supports_unconstrained_layout() {
        let lines = layout_text_with("alpha beta\ngamma", 0.0, width);

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text, "alpha beta");
        assert!(lines[0].is_hard_break);
        assert_eq!(lines[1].text, "gamma");
    }

    #[test]
    fn counts_words_and_paragraphs() {
        assert_eq!(count_words("alpha beta  gamma"), 3);
        assert_eq!(paragraph_count("alpha\nbeta\n"), 3);
        assert_eq!(paragraph_count(""), 0);
    }

    #[test]
    fn summarizes_layout() {
        let lines = layout_text_with("alpha beta gamma", 10.0, width);
        let summary = summarize_layout(&lines, 18.0);

        assert_eq!(summary.line_count, 2);
        assert_eq!(summary.max_line_width, 10.0);
        assert_eq!(summary.total_height, 36.0);
        assert_eq!(summary.paragraph_count, 1);
    }

    #[test]
    fn normalizes_invalid_line_height() {
        assert_eq!(normalized_line_height(0.0), 16.0);
        assert_eq!(normalized_line_height(-2.0), 16.0);
        assert_eq!(normalized_line_height(24.0), 24.0);
    }
}
