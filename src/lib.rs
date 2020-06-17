use mandelbrot_common::generate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
pub fn draw(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    n: u32,
    max_steps: u8
) -> Result<(), JsValue> {
    let (mut data, width, height) = get_mandelbrot_data(n, max_steps);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    canvas.set_width(width);
    canvas.set_height(height);
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_mandelbrot_data(n: u32, max_steps: u8) -> (Vec<u8>, u32, u32) {
    let mut data = Vec::new();
    let raw_set_data = generate(n as usize, 4f64, max_steps);
    let min = raw_set_data.iter().map(|x| *x).min().unwrap();
    let max = raw_set_data.iter().map(|x| *x).max().unwrap();
    console::log_1(&format!("width: {}, height: {}", raw_set_data.ncols(), raw_set_data.nrows()).into());
    raw_set_data.iter().for_each(|x| {
        let rgb = heat_map(min, max, *x);
        data.push(rgb.red);
        data.push(rgb.green);
        data.push(rgb.blue);
        data.push(255);
    });
    (data, raw_set_data.ncols() as u32, raw_set_data.nrows() as u32)
}

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn heat_map(min: u8, max: u8, value: u8) -> RGB {
    let ratio: f64 = 2f64 * (value as f64 - min as f64) / (max as f64 - min as f64);
    let b = 255f64 * (1f64 - ratio);
    let r = 255f64 * (ratio - 1f64);
    let blue= if b > 0.0 { b as u8 } else { 0 };
    let red = if r > 0.0 { r as u8 } else { 0 };
    let green = 255 - blue - red;
    RGB { red, green, blue }
}