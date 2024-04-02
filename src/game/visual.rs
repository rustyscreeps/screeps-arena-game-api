use crate::{game::pathfinder::Position, traits::GameObjectProperties};
use js_sys::Array;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct VisualPosition {
    pub x: f32,
    pub y: f32,
}

impl VisualPosition {
    pub fn offset(&self, x: f32, y: f32) -> VisualPosition {
        VisualPosition {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl From<&Position> for VisualPosition {
    fn from(pos: &Position) -> Self {
        VisualPosition {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}

impl From<Position> for VisualPosition {
    fn from(pos: Position) -> Self {
        VisualPosition {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}

impl<T> From<T> for VisualPosition
where
    T: GameObjectProperties,
{
    fn from(obj: T) -> Self {
        VisualPosition {
            x: obj.x() as f32,
            y: obj.y() as f32,
        }
    }
}

impl From<VisualPosition> for JsValue {
    fn from(pos: VisualPosition) -> JsValue {
        serde_wasm_bindgen::to_value(&pos).expect("serializable VisualPosition")
    }
}

impl From<&VisualPosition> for JsValue {
    fn from(pos: &VisualPosition) -> JsValue {
        serde_wasm_bindgen::to_value(pos).expect("serializable VisualPosition")
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CircleStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f32>,
}

impl CircleStyle {
    pub fn radius(mut self, val: f32) -> CircleStyle {
        self.radius = Some(val);
        self
    }

    pub fn fill(mut self, val: &str) -> CircleStyle {
        self.fill = Some(val.to_string());
        self
    }

    pub fn opacity(mut self, val: f32) -> CircleStyle {
        self.opacity = Some(val);
        self
    }

    pub fn stroke(mut self, val: &str) -> CircleStyle {
        self.stroke = Some(val.to_string());
        self
    }

    pub fn stroke_width(mut self, val: f32) -> CircleStyle {
        self.stroke_width = Some(val);
        self
    }
}

impl From<CircleStyle> for JsValue {
    fn from(style: CircleStyle) -> JsValue {
        serde_wasm_bindgen::to_value(&style).expect("serializable CircleStyle")
    }
}

impl From<&CircleStyle> for JsValue {
    fn from(circle: &CircleStyle) -> JsValue {
        serde_wasm_bindgen::to_value(circle).expect("serializable CircleStyle")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LineDrawStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl LineDrawStyle {
    pub fn is_solid(&self) -> bool {
        matches!(self, LineDrawStyle::Solid)
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
    #[serde(skip_serializing_if = "LineDrawStyle::is_solid")]
    line_style: LineDrawStyle,
}

impl LineStyle {
    pub fn width(mut self, val: f32) -> LineStyle {
        self.width = Some(val);
        self
    }

    pub fn color(mut self, val: &str) -> LineStyle {
        self.color = Some(val.to_string());
        self
    }

    pub fn opacity(mut self, val: f32) -> LineStyle {
        self.opacity = Some(val);
        self
    }

    pub fn line_style(mut self, val: LineDrawStyle) -> LineStyle {
        self.line_style = val;
        self
    }
}

impl From<LineStyle> for JsValue {
    fn from(style: LineStyle) -> JsValue {
        serde_wasm_bindgen::to_value(&style).expect("serializable LineStyle")
    }
}

impl From<&LineStyle> for JsValue {
    fn from(style: &LineStyle) -> JsValue {
        serde_wasm_bindgen::to_value(style).expect("serializable LineStyle")
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RectStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f32>,
    #[serde(skip_serializing_if = "LineDrawStyle::is_solid")]
    line_style: LineDrawStyle,
}

impl RectStyle {
    pub fn fill(mut self, val: &str) -> RectStyle {
        self.fill = Some(val.to_string());
        self
    }

    pub fn opacity(mut self, val: f32) -> RectStyle {
        self.opacity = Some(val);
        self
    }

    pub fn stroke(mut self, val: &str) -> RectStyle {
        self.stroke = Some(val.to_string());
        self
    }

    pub fn stroke_width(mut self, val: f32) -> RectStyle {
        self.stroke_width = Some(val);
        self
    }

    pub fn line_style(mut self, val: LineDrawStyle) -> RectStyle {
        self.line_style = val;
        self
    }
}

impl From<RectStyle> for JsValue {
    fn from(style: RectStyle) -> JsValue {
        serde_wasm_bindgen::to_value(&style).expect("serializable RectStyle")
    }
}

impl From<&RectStyle> for JsValue {
    fn from(style: &RectStyle) -> JsValue {
        serde_wasm_bindgen::to_value(style).expect("serializable RectStyle")
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolyStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f32>,
    #[serde(skip_serializing_if = "LineDrawStyle::is_solid")]
    line_style: LineDrawStyle,
}

impl PolyStyle {
    pub fn fill(mut self, val: &str) -> PolyStyle {
        self.fill = Some(val.to_string());
        self
    }

    pub fn opacity(mut self, val: f32) -> PolyStyle {
        self.opacity = Some(val);
        self
    }

    pub fn stroke(mut self, val: &str) -> PolyStyle {
        self.stroke = Some(val.to_string());
        self
    }

    pub fn stroke_width(mut self, val: f32) -> PolyStyle {
        self.stroke_width = Some(val);
        self
    }

    pub fn line_style(mut self, val: LineDrawStyle) -> PolyStyle {
        self.line_style = val;
        self
    }
}

impl From<PolyStyle> for JsValue {
    fn from(style: PolyStyle) -> JsValue {
        serde_wasm_bindgen::to_value(&style).expect("serializable PolyStyle")
    }
}

impl From<&PolyStyle> for JsValue {
    fn from(style: &PolyStyle) -> JsValue {
        serde_wasm_bindgen::to_value(style).expect("serializable PolyStyle")
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum FontStyle {
    Size(f32),
    Custom(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TextAlign {
    #[default]
    Center,
    Left,
    Right,
}

impl TextAlign {
    pub fn is_center(&self) -> bool {
        matches!(self, TextAlign::Center)
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<FontStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_width: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_padding: Option<f32>,
    #[serde(skip_serializing_if = "TextAlign::is_center")]
    align: TextAlign,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
}

impl TextStyle {
    pub fn color(mut self, val: &str) -> TextStyle {
        self.color = Some(val.to_string());
        self
    }

    pub fn font_size(mut self, val: f32) -> TextStyle {
        self.font = Some(FontStyle::Size(val));
        self
    }

    pub fn font_style(mut self, val: &str) -> TextStyle {
        self.font = Some(FontStyle::Custom(val.to_string()));
        self
    }

    pub fn stroke(mut self, val: &str) -> TextStyle {
        self.stroke = Some(val.to_string());
        self
    }

    pub fn stroke_width(mut self, val: f32) -> TextStyle {
        self.stroke_width = Some(val);
        self
    }

    pub fn background_color(mut self, val: &str) -> TextStyle {
        self.background_color = Some(val.to_string());
        self
    }

    pub fn background_padding(mut self, val: f32) -> TextStyle {
        self.background_padding = Some(val);
        self
    }

    pub fn align(mut self, val: TextAlign) -> TextStyle {
        self.align = val;
        self
    }

    pub fn opacity(mut self, val: f32) -> TextStyle {
        self.opacity = Some(val);
        self
    }
}

impl From<TextStyle> for JsValue {
    fn from(style: TextStyle) -> JsValue {
        serde_wasm_bindgen::to_value(&style).expect("serializable TextStyle")
    }
}

impl From<&TextStyle> for JsValue {
    fn from(style: &TextStyle) -> JsValue {
        serde_wasm_bindgen::to_value(style).expect("serializable TextStyle")
    }
}

#[wasm_bindgen(module = "game/visual")]
extern "C" {
    #[wasm_bindgen]
    pub type Visual;

    #[wasm_bindgen(constructor)]
    pub fn new(layer: Option<i32>, persistent: bool) -> Visual;

    #[wasm_bindgen(method, getter)]
    pub fn layer(this: &Visual) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn persistent(this: &Visual) -> bool;

    #[wasm_bindgen(method, js_name = circle)]
    pub fn circle_internal(this: &Visual, pos: &JsValue, style: &JsValue) -> Visual;

    #[wasm_bindgen(method, js_name = line)]
    pub fn line_internal(this: &Visual, from: &JsValue, to: &JsValue, style: &JsValue) -> Visual;

    #[wasm_bindgen(method, js_name = rect)]
    pub fn rect_internal(
        this: &Visual,
        top_left: &JsValue,
        width: f32,
        height: f32,
        style: &JsValue,
    ) -> Visual;

    #[wasm_bindgen(method, js_name = poly)]
    pub fn poly_internal(this: &Visual, points: &Array, style: &JsValue) -> Visual;

    #[wasm_bindgen(method, js_name = text)]
    pub fn text_internal(this: &Visual, text: &str, pos: &JsValue, style: &JsValue) -> Visual;
}

impl Visual {
    pub fn circle(self: &Visual, pos: &VisualPosition, style: Option<&CircleStyle>) -> Visual {
        match style {
            Some(style) => self.circle_internal(&JsValue::from(pos), &JsValue::from(style)),
            None => self.circle_internal(&JsValue::from(pos), &JsValue::UNDEFINED),
        }
    }

    pub fn line(
        self: &Visual,
        from: &VisualPosition,
        to: &VisualPosition,
        style: Option<&LineStyle>,
    ) -> Visual {
        match style {
            Some(style) => self.line_internal(
                &JsValue::from(from),
                &JsValue::from(to),
                &JsValue::from(style),
            ),
            None => self.line_internal(
                &JsValue::from(from),
                &JsValue::from(to),
                &JsValue::UNDEFINED,
            ),
        }
    }

    pub fn rect(
        self: &Visual,
        top_left: &VisualPosition,
        width: f32,
        height: f32,
        style: Option<&RectStyle>,
    ) -> Visual {
        match style {
            Some(style) => self.rect_internal(
                &JsValue::from(top_left),
                width,
                height,
                &JsValue::from(style),
            ),
            None => {
                self.rect_internal(&JsValue::from(top_left), width, height, &JsValue::UNDEFINED)
            }
        }
    }

    pub fn poly(self: &Visual, points: &[VisualPosition], style: Option<&PolyStyle>) -> Visual {
        let points = points.iter().cloned().map(JsValue::from).collect();
        match style {
            Some(style) => self.poly_internal(&points, &JsValue::from(style)),
            None => self.poly_internal(&points, &JsValue::UNDEFINED),
        }
    }

    pub fn text(
        self: &Visual,
        text: &str,
        pos: &VisualPosition,
        style: Option<&TextStyle>,
    ) -> Visual {
        match style {
            Some(style) => self.text_internal(text, &JsValue::from(pos), &JsValue::from(style)),

            None => self.text_internal(text, &JsValue::from(pos), &JsValue::UNDEFINED),
        }
    }
}
