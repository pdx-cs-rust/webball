use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// https://robertsnakard.com/20191130_tetris_4.html
use gloo_timers::callback::Interval;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

 

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    let r = 3.0;
    let mut x = 1.0;
    let mut y = 1.0;
    let mut dx = 1.0;
    let mut dy = 1.0;

    let render = move || {
        // Erase the ball.
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("white"));
        context
            .fill_rect(x - 1.0, y - 1.0, 2.0 * r + 2.0, 2.0 * r + 2.0);
        context.fill();

        // Move the ball.
        if x <= 0.0 || x >= width - 2.0 * r {
            dx = -dx;
        }
        if y <= 0.0 || y >= height - 2.0 * r {
            dy = -dy;
        }
        x += dx;
        y += dy;

        // Draw the frame.
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str("black"));
        context.set_line_width(2.0);
        context.rect(0.5, 0.5, width - 1.5, height - 1.5);
        context.stroke();

        // Draw the ball.
        context.begin_path();
        context.set_fill_style(&JsValue::from_str("red"));
        context
            .arc(x + r, y + r, r, 0.0, f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
    };

    Interval::new(20, render).forget();
}
